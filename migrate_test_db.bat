@echo off
chcp 65001 >nul
echo ========================================
echo   测试数据库迁移
echo ========================================
echo.

echo [1/2] 创建测试数据库（如果不存在）...
mysql -u app_user -p"AppPass123!" -h localhost -e "CREATE DATABASE IF NOT EXISTS campus_test CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci;"

if errorlevel 1 (
    echo ❌ 创建数据库失败
    pause
    exit /b 1
)

echo ✅ 测试数据库已就绪
echo.

echo [2/2] 运行迁移到 campus_test...
sqlx migrate run --database-url "mysql://app_user:AppPass123!@localhost:3306/campus_test"

if errorlevel 1 (
    echo ❌ 迁移失败
    pause
    exit /b 1
)

echo.
echo ✅ 迁移成功！
echo.

echo 验证表结构...
mysql -u app_user -p"AppPass123!" -h localhost campus_test -e "SHOW TABLES;"

echo.
echo ========================================
echo   测试数据库迁移完成
echo ========================================
echo.
pause
