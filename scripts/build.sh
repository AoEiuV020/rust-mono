#!/bin/bash

# Rust 多模块项目构建脚本 - 动态库版本
# 对比静态链接 vs 动态链接的差异

set -e  # 遇到错误立即退出

# 颜色定义
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# 项目根目录
PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
BUILD_DIR="${PROJECT_ROOT}/build"

echo -e "${BLUE}========================================${NC}"
echo -e "${BLUE}Rust 多模块项目构建脚本 (动态库版本)${NC}"
echo -e "${BLUE}========================================${NC}"

# 清理之前的构建产物
echo -e "\n${YELLOW}[1/5] 清理构建目录...${NC}"
rm -rf "${BUILD_DIR}"
mkdir -p "${BUILD_DIR}"/{static,dynamic/lib}

# 检测操作系统
OS_TYPE=$(uname)
if [ "$OS_TYPE" = "Darwin" ]; then
    LIB_EXT="dylib"
    echo "  - 检测到 macOS，将生成 .dylib 文件"
else
    LIB_EXT="so"
    echo "  - 检测到 Linux，将生成 .so 文件"
fi

# 编译静态链接应用
echo -e "\n${YELLOW}[2/5] 编译静态链接应用...${NC}"
echo "  - 使用标准 cargo build --release，直接依赖三个库"
cd "${PROJECT_ROOT}"
cargo build --release --package static-app
cp target/release/static-app "${BUILD_DIR}/static/"
echo -e "${GREEN}  ✓ 静态链接应用编译完成${NC}"

# 编译动态库
echo -e "\n${YELLOW}[3/5] 编译动态库（.${LIB_EXT}）...${NC}"

# 编译 common 动态库
echo "  - 编译 libcommon.${LIB_EXT}..."
cargo build --release --package common
cp "target/release/libcommon.${LIB_EXT}" "${BUILD_DIR}/dynamic/lib/"
echo -e "${GREEN}    ✓ libcommon.${LIB_EXT} 编译完成${NC}"

# 编译 mathlib 动态库
echo "  - 编译 libmathlib.${LIB_EXT}..."
cargo build --release --package mathlib
cp "target/release/libmathlib.${LIB_EXT}" "${BUILD_DIR}/dynamic/lib/"
echo -e "${GREEN}    ✓ libmathlib.${LIB_EXT} 编译完成${NC}"

# 编译 stringlib 动态库
echo "  - 编译 libstringlib.${LIB_EXT}..."
cargo build --release --package stringlib
cp "target/release/libstringlib.${LIB_EXT}" "${BUILD_DIR}/dynamic/lib/"
echo -e "${GREEN}    ✓ libstringlib.${LIB_EXT} 编译完成${NC}"

# 编译动态链接应用
echo -e "\n${YELLOW}[4/5] 编译动态链接应用...${NC}"
echo "  - 使用 libloading 加载动态库"
cargo build --release --package dynamic-app
cp target/release/dynamic-app "${BUILD_DIR}/dynamic/"
echo -e "${GREEN}  ✓ 动态链接应用编译完成${NC}"

# 在 macOS 上修改动态库加载路径
if [ "$OS_TYPE" = "Darwin" ]; then
    echo "  - 修改 rpath 为相对路径..."
    install_name_tool -add_rpath "@executable_path/lib" "${BUILD_DIR}/dynamic/dynamic-app" 2>/dev/null || true
    
    # 修改动态库的依赖路径
    for lib in "${BUILD_DIR}/dynamic/lib"/*.dylib; do
        if [ -f "$lib" ]; then
            LIB_NAME=$(basename "$lib")
            # 修改库的 id
            install_name_tool -id "@rpath/${LIB_NAME}" "$lib" 2>/dev/null || true
            # 修改对 common 库的依赖
            install_name_tool -change \
                "${PROJECT_ROOT}/target/release/libcommon.dylib" \
                "@rpath/libcommon.dylib" \
                "$lib" 2>/dev/null || true
        fi
    done
fi

# 创建运行脚本
echo -e "\n${YELLOW}[5/5] 创建运行脚本...${NC}"

# 静态链接应用运行脚本
cat > "${BUILD_DIR}/static/run.sh" << 'EOF'
#!/bin/bash
cd "$(dirname "$0")"
./static-app
EOF
chmod +x "${BUILD_DIR}/static/run.sh"
echo -e "${GREEN}  ✓ 静态链接运行脚本创建完成${NC}"

# 动态链接应用运行脚本
cat > "${BUILD_DIR}/dynamic/run.sh" << 'EOF'
#!/bin/bash
cd "$(dirname "$0")"
# 设置动态库搜索路径
export DYLD_LIBRARY_PATH="$(pwd)/lib:${DYLD_LIBRARY_PATH}"
export LD_LIBRARY_PATH="$(pwd)/lib:${LD_LIBRARY_PATH}"
./dynamic-app
EOF
chmod +x "${BUILD_DIR}/dynamic/run.sh"
echo -e "${GREEN}  ✓ 动态链接运行脚本创建完成${NC}"

# 显示文件大小统计
echo -e "\n${BLUE}========================================${NC}"
echo -e "${BLUE}文件大小统计${NC}"
echo -e "${BLUE}========================================${NC}"

echo -e "\n${YELLOW}静态链接版本:${NC}"
if [ -f "${BUILD_DIR}/static/static-app" ]; then
    STATIC_SIZE=$(stat -f%z "${BUILD_DIR}/static/static-app" 2>/dev/null || stat -c%s "${BUILD_DIR}/static/static-app" 2>/dev/null)
    STATIC_SIZE_H=$(ls -lh "${BUILD_DIR}/static/static-app" | awk '{print $5}')
    echo "  static-app: ${STATIC_SIZE_H} (${STATIC_SIZE} bytes)"
    echo "  总计: ${STATIC_SIZE_H}"
fi

echo -e "\n${YELLOW}动态链接版本:${NC}"
DYNAMIC_TOTAL=0
if [ -f "${BUILD_DIR}/dynamic/dynamic-app" ]; then
    APP_SIZE=$(stat -f%z "${BUILD_DIR}/dynamic/dynamic-app" 2>/dev/null || stat -c%s "${BUILD_DIR}/dynamic/dynamic-app" 2>/dev/null)
    APP_SIZE_H=$(ls -lh "${BUILD_DIR}/dynamic/dynamic-app" | awk '{print $5}')
    echo "  dynamic-app: ${APP_SIZE_H} (${APP_SIZE} bytes)"
    DYNAMIC_TOTAL=$((DYNAMIC_TOTAL + APP_SIZE))
fi

echo "  动态库:"
for lib in "${BUILD_DIR}/dynamic/lib"/*.${LIB_EXT}; do
    if [ -f "$lib" ]; then
        LIB_SIZE=$(stat -f%z "$lib" 2>/dev/null || stat -c%s "$lib" 2>/dev/null)
        LIB_SIZE_H=$(ls -lh "$lib" | awk '{print $5}')
        LIB_NAME=$(basename "$lib")
        echo "    ${LIB_NAME}: ${LIB_SIZE_H} (${LIB_SIZE} bytes)"
        DYNAMIC_TOTAL=$((DYNAMIC_TOTAL + LIB_SIZE))
    fi
done

# 计算总计
DYNAMIC_TOTAL_H=$(numfmt --to=iec-i --suffix=B $DYNAMIC_TOTAL 2>/dev/null || echo "$((DYNAMIC_TOTAL / 1024))K")
echo "  总计: ${DYNAMIC_TOTAL_H} (${DYNAMIC_TOTAL} bytes)"

# 计算差异
if [ -n "$STATIC_SIZE" ] && [ -n "$DYNAMIC_TOTAL" ]; then
    echo -e "\n${YELLOW}对比分析:${NC}"
    DIFF=$((STATIC_SIZE - DYNAMIC_TOTAL))
    
    if [ $DIFF -gt 0 ]; then
        DIFF_H=$(numfmt --to=iec-i --suffix=B $DIFF 2>/dev/null || echo "$((DIFF / 1024))K")
        PERCENT=$(awk "BEGIN {printf \"%.2f\", (($DIFF * 1.0) / $STATIC_SIZE) * 100}")
        echo "  静态链接比动态链接小: ${DIFF_H} (${PERCENT}%)"
    elif [ $DIFF -lt 0 ]; then
        DIFF_ABS=$((-DIFF))
        DIFF_H=$(numfmt --to=iec-i --suffix=B $DIFF_ABS 2>/dev/null || echo "$((DIFF_ABS / 1024))K")
        PERCENT=$(awk "BEGIN {printf \"%.2f\", (($DIFF_ABS * 1.0) / $DYNAMIC_TOTAL) * 100}")
        echo "  动态链接比静态链接大: ${DIFF_H} (${PERCENT}%)"
    else
        echo "  两者大小相同"
    fi
fi

echo -e "\n${GREEN}========================================${NC}"
echo -e "${GREEN}构建成功完成！${NC}"
echo -e "${GREEN}========================================${NC}"
echo ""
echo "运行静态链接应用: ${BUILD_DIR}/static/run.sh"
echo "运行动态链接应用: ${BUILD_DIR}/dynamic/run.sh"
echo ""
