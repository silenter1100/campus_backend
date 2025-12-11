#!/usr/bin/env python3
"""
API 测试脚本（支持 JWT 认证和模块化测试）
使用方法：
1. 安装依赖：pip install requests protobuf
2. 编译 proto：protoc --python_out=. proto/course.proto
3. 运行测试：
   - 全部测试：python test_api.py
   - 课表模块：python test_api.py course
   - 用户模块：python test_api.py user
   - 指定用户：python test_api.py [module] [user_id]
"""

import requests
import sys
import os
import json

# 添加生成的 proto 文件路径
sys.path.insert(0, os.path.dirname(__file__))

try:
    from proto import course_pb2, user_pb2
    from generate_token import generate_jwt_token
except ImportError as e:
    print(f"错误：找不到必要模块: {e}")
    print("请确保：")
    print("1. 运行：protoc --python_out=. proto/course.proto")
    print("2. 运行：protoc --python_out=. proto/user.proto")
    print("3. generate_token.py 文件存在")
    sys.exit(1)

BASE_URL = "http://localhost:3000"

# JWT 认证配置
TEST_USER_ID = "1"  # 测试用户 ID，改为字符串类型
JWT_TOKEN = None  # 全局 token 变量
TEST_MODULE = "all"  # 测试模块：all, course, user

def set_test_user(user_id):
    """切换测试用户"""
    global TEST_USER_ID, JWT_TOKEN
    TEST_USER_ID = str(user_id)
    JWT_TOKEN = None  # 重置 token，下次请求时会重新生成
    print(f"🔄 切换到用户 ID: {user_id}")

def set_test_module(module):
    """设置测试模块"""
    global TEST_MODULE
    TEST_MODULE = module
    print(f"📋 测试模块: {module}")

def get_auth_headers():
    """获取带认证的请求头"""
    global JWT_TOKEN
    if JWT_TOKEN is None:
        try:
            JWT_TOKEN = generate_jwt_token(TEST_USER_ID)
            print(f"🔑 为用户 {TEST_USER_ID} 生成 JWT Token")
        except Exception as e:
            print(f"❌ 生成 JWT Token 失败: {e}")
            return {}
    
    return {
        'Authorization': f'Bearer {JWT_TOKEN}',
        'Accept': 'application/x-protobuf'
    }

def get_public_headers():
    """获取公开接口的请求头（无需认证）"""
    return {'Accept': 'application/x-protobuf'}

def get_json_headers():
    """获取JSON格式的请求头（用于用户模块）"""
    global JWT_TOKEN
    if JWT_TOKEN is None:
        try:
            JWT_TOKEN = generate_jwt_token(TEST_USER_ID)
            print(f"🔑 为用户 {TEST_USER_ID} 生成 JWT Token")
        except Exception as e:
            print(f"❌ 生成 JWT Token 失败: {e}")
            return {}
    
    return {
        'Authorization': f'Bearer {JWT_TOKEN}',
        'Content-Type': 'application/json',
        'Accept': 'application/json'
    }

def print_separator(title):
    print("\n" + "="*60)
    print(f"  {title}")
    print("="*60)

def test_get_semesters():
    """测试获取学期列表（公开接口）"""
    print_separator("测试：获取学期列表（公开接口）")
    
    response = requests.get(
        f"{BASE_URL}/api/v1/semesters",
        headers=get_public_headers()
    )
    
    print(f"状态码: {response.status_code}")
    print(f"Content-Type: {response.headers.get('Content-Type')}")
    
    if response.status_code == 200:
        result = course_pb2.GetSemestersResponse()
        result.ParseFromString(response.content)
        
        print(f"\n响应码: {result.code}")
        print(f"消息: {result.message}")
        print(f"\n学期列表 (共 {len(result.data.semesters)} 个):")
        for semester in result.data.semesters:
            current = "✓ 当前学期" if semester.is_current else ""
            print(f"  - ID: {semester.id}, 名称: {semester.name} {current}")
            print(f"    时间: {semester.start_date} ~ {semester.end_date}")
    else:
        print(f"请求失败: {response.text}")

def test_get_public_courses(semester_id=None, page=1, page_size=5):
    """测试获取全校课程（公开接口）"""
    print_separator("测试：获取全校课程（公开接口）")
    
    params = {
        'page': page,
        'pageSize': page_size
    }
    if semester_id:
        params['semester_id'] = semester_id
    
    response = requests.get(
        f"{BASE_URL}/api/v1/courses",
        params=params,
        headers=get_public_headers()
    )
    
    print(f"状态码: {response.status_code}")
    
    if response.status_code == 200:
        result = course_pb2.GetPublicCoursesResponse()
        result.ParseFromString(response.content)
        
        print(f"\n响应码: {result.code}")
        print(f"消息: {result.message}")
        print(f"\n分页信息:")
        print(f"  总数: {result.data.pagination.total}")
        print(f"  当前页: {result.data.pagination.page}/{result.data.pagination.pages}")
        print(f"\n课程列表:")
        for course in result.data.list:
            print(f"  - {course.course_name} ({course.teacher_name})")
            print(f"    时间: 周{course.day_of_week} 第{course.start_section}-{course.end_section}节")
            print(f"    地点: {course.location}")
    else:
        print(f"请求失败: {response.text}")

def test_get_schedule(semester_id, week=None):
    """测试获取用户课表（需要认证）"""
    print_separator("测试：获取用户课表（需要认证）")
    
    params = {'semester_id': semester_id}
    if week:
        params['week'] = week
    
    response = requests.get(
        f"{BASE_URL}/api/v1/schedule",
        params=params,
        headers=get_auth_headers()
    )
    
    print(f"状态码: {response.status_code}")
    
    if response.status_code == 200:
        result = course_pb2.GetScheduleResponse()
        result.ParseFromString(response.content)
        
        print(f"\n响应码: {result.code}")
        print(f"消息: {result.message}")
        print(f"\n课表项 (共 {len(result.data.items)} 项):")
        for item in result.data.items:
            custom = "✓ 自定义" if item.is_custom else ""
            print(f"  - {item.course_name} ({item.teacher_name}) {custom}")
            print(f"    时间: 周{item.day_of_week} 第{item.start_section}-{item.end_section}节")
            print(f"    地点: {item.location}")
            if item.color_hex:
                print(f"    颜色: {item.color_hex}")
    else:
        print(f"请求失败: {response.text}")

def test_add_schedule_items(semester_id):
    """测试添加课表项（需要认证）"""
    print_separator("测试：添加课表项（需要认证）")
    
    # 构造请求
    request = course_pb2.AddScheduleItemsRequest(
        semester_id=semester_id,
        items=[
            course_pb2.ScheduleItemInput(
                source_id=1,                   
                course_name="高等数学A",
                teacher_name="张教授",
                location="教学楼A-101",
                day_of_week=1,
                start_section=1,
                end_section=2,
                weeks=[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16],
                type="compulsory",              
                credits=4,
                description="高等数学基础课程",
                color_hex="#000000",            # 默认颜色（可选）
                is_custom=False                 # 默认非自定义
            ),
            course_pb2.ScheduleItemInput(
                course_name="自定义课程B",
                teacher_name="李老师",
                location="图书馆",
                day_of_week=6,
                start_section=5,
                end_section=6,
                weeks=[10],
                type="选修",
                credits=2,
                color_hex="#33FF57",
                is_custom=True
            ),
            course_pb2.ScheduleItemInput(
                course_name="测试课程B",  # 新课程名称
                teacher_name="李老师",  # 假设是另一位教师
                location="教学楼A101",  # 同一地点
                day_of_week=1,  # 星期一
                start_section=1,  # 开始节次相同
                end_section=2,  # 结束节次相同
                weeks=[8, 9, 11, 12, 13, 14],  # 不重合的周数组
                type="选修",  # 或者"必修"，根据实际需求
                credits=2,  # 学分可以根据实际情况调整
                description="这是另一个测试课程",
                color_hex="#33FF57",  # 不同颜色以区分
                is_custom=True
            )
        ]
    )
    
    # 获取认证头并添加 Content-Type
    headers = get_auth_headers()
    headers['Content-Type'] = 'application/x-protobuf'
    
    response = requests.post(
        f"{BASE_URL}/api/v1/schedule",
        headers=headers,
        data=request.SerializeToString()
    )
    
    print(f"状态码: {response.status_code}")
    
    if response.status_code == 200:
        result = course_pb2.AddScheduleItemsResponse()
        result.ParseFromString(response.content)
        
        print(f"\n响应码: {result.code}")
        print(f"消息: {result.message}")
        print(f"\n成功添加 {len(result.data.successful_items)} 项:")
        for item in result.data.successful_items:
            print(f"  - ID: {item.id}, 课程: {item.course_name}")
        
        if result.data.failed_items:
            print(f"\n失败 {len(result.data.failed_items)} 项:")
            for item in result.data.failed_items:
                print(f"  - 课程: {item.course_name}, 错误: {item.error_message}")
    else:
        print(f"请求失败: {response.text}")

def test_update_schedule_item(item_id):
    """测试更新课表项（需要认证）"""
    print_separator("测试：更新课表项（需要认证）")
    
    request = course_pb2.UpdateScheduleItemRequest(
        course_name="更新后的课程名",
        teacher_name="王老师",
        location="新教学楼B202",
        day_of_week=2,
        start_section=3,
        end_section=4,
        weeks=[1,2,3,4,5,6,7,8,10],
        type="必修",
        credits=4,
        description="课程已更新",
        color_hex="#3357FF"
    )
    
    # 获取认证头并添加 Content-Type
    headers = get_auth_headers()
    headers['Content-Type'] = 'application/x-protobuf'
    
    response = requests.patch(
        f"{BASE_URL}/api/v1/schedule",
        params={'item_id': item_id},
        headers=headers,
        data=request.SerializeToString()
    )
    
    print(f"状态码: {response.status_code}")
    
    if response.status_code == 200:
        result = course_pb2.UpdateScheduleItemResponse()
        result.ParseFromString(response.content)
        
        print(f"\n响应码: {result.code}")
        print(f"消息: {result.message}")
        print(f"\n更新后的课表项:")
        item = result.data.item
        print(f"  - ID: {item.id}")
        print(f"  - 课程: {item.course_name} ({item.teacher_name})")
        print(f"  - 时间: 周{item.day_of_week} 第{item.start_section}-{item.end_section}节")
    else:
        print(f"请求失败: {response.text}")

def test_delete_schedule_item(item_id):
    """测试删除课表项（需要认证）"""
    print_separator("测试：删除课表项（需要认证）")
    
    response = requests.delete(
        f"{BASE_URL}/api/v1/schedule",
        params={'item_id': item_id},
        headers=get_auth_headers()
    )
    
    print(f"状态码: {response.status_code}")
    
    if response.status_code == 200:
        result = course_pb2.DeleteScheduleItemResponse()
        result.ParseFromString(response.content)
        
        print(f"\n响应码: {result.code}")
        print(f"消息: {result.message}")
    else:
        print(f"请求失败: {response.text}")

def test_unauthorized_access():
    """测试无认证访问（应该失败）"""
    print_separator("测试：无认证访问（应该返回 401）")
    
    response = requests.get(
        f"{BASE_URL}/api/v1/schedule",
        params={'semester_id': 1},
        headers={'Accept': 'application/x-protobuf'}  # 不包含 Authorization 头
    )
    
    print(f"状态码: {response.status_code}")
    if response.status_code == 401:
        print("✅ 正确：未认证请求被拒绝")
    else:
        print("❌ 错误：未认证请求应该返回 401")
    
    if response.content:
        print(f"响应内容: {response.content}")

# ==================== 用户模块测试函数 ====================

def test_user_login():
    """测试用户登录"""
    print_separator("测试：用户登录")
    
    # 构造Protobuf请求
    request = user_pb2.LoginRequest(
        student_id="2021001001",
        password="password123"
    )
    
    response = requests.post(
        f"{BASE_URL}/api/v1/auth/login",
        headers={'Content-Type': 'application/x-protobuf', 'Accept': 'application/x-protobuf'},
        data=request.SerializeToString()
    )
    
    print(f"状态码: {response.status_code}")
    
    if response.status_code == 200:
        result = user_pb2.LoginResponse()
        result.ParseFromString(response.content)
        
        print(f"响应码: {result.code}")
        print(f"消息: {result.message}")
        
        if result.data and result.data.token:
            global JWT_TOKEN
            JWT_TOKEN = result.data.token
            print(f"✅ 登录成功，获得 Token: {JWT_TOKEN[:50]}...")
            
            user_info = result.data.user
            print(f"\n用户信息:")
            print(f"  - ID: {user_info.id}")
            print(f"  - 学号: {user_info.student_id}")
            print(f"  - 姓名: {user_info.name}")
            print(f"  - 学院: {user_info.college}")
            print(f"  - 专业: {user_info.major}")
            print(f"  - 角色: {user_info.role}")
        else:
            print("❌ 登录响应格式错误")
    else:
        print(f"❌ 登录失败: {response.text}")

def test_user_register():
    """测试用户注册"""
    print_separator("测试：用户注册")
    
    # 生成测试用户数据
    import time
    timestamp = int(time.time())
    
    request = user_pb2.RegisterRequest(
        student_id=f"test{timestamp}",
        password="testpass123",
        name=f"测试用户{timestamp}",
        college="测试学院",
        major="测试专业",
        phone=f"138{timestamp % 100000000:08d}"
    )
    
    response = requests.post(
        f"{BASE_URL}/api/v1/auth/register",
        headers={'Content-Type': 'application/x-protobuf', 'Accept': 'application/x-protobuf'},
        data=request.SerializeToString()
    )
    
    print(f"状态码: {response.status_code}")
    
    if response.status_code == 200:
        result = user_pb2.RegisterResponse()
        result.ParseFromString(response.content)
        
        print(f"响应码: {result.code}")
        print(f"消息: {result.message}")
        
        if result.data and result.data.user_id:
            print(f"✅ 注册成功，用户ID: {result.data.user_id}")
        else:
            print("❌ 注册响应格式错误")
    else:
        print(f"❌ 注册失败: {response.text}")

def test_get_user_info():
    """测试获取用户信息"""
    print_separator("测试：获取用户信息")
    
    response = requests.get(
        f"{BASE_URL}/api/v1/users/me",
        headers=get_auth_headers()
    )
    
    print(f"状态码: {response.status_code}")
    
    if response.status_code == 200:
        result = user_pb2.GetUserInfoResponse()
        result.ParseFromString(response.content)
        
        print(f"响应码: {result.code}")
        print(f"消息: {result.message}")
        
        if result.data:
            user_info = result.data
            print(f"\n用户详细信息:")
            print(f"  - ID: {user_info.id}")
            print(f"  - 学号: {user_info.student_id}")
            print(f"  - 姓名: {user_info.name}")
            print(f"  - 学院: {user_info.college}")
            print(f"  - 专业: {user_info.major}")
            print(f"  - 班级: {user_info.class_name}")
            print(f"  - 电话: {user_info.phone}")
            print(f"  - 邮箱: {user_info.email}")
            print(f"  - 角色: {user_info.role}")
            print(f"  - 年级: {user_info.grade}")
            print(f"  - 个人简介: {user_info.bio}")
            print(f"  - 微信号: {user_info.wechat_id}")
            print(f"  - 收藏数: {user_info.collection_count}")
            print(f"  - 论坛活跃度: {user_info.forum_activity_score}")
            print(f"  - 本周课时数: {user_info.weekly_course_count}")
            print(f"  - 课表隐私设置: {user_info.setting_privacy_course}")
            print(f"  - 通知开关: {user_info.setting_notification_switch}")
        else:
            print("❌ 获取用户信息响应格式错误")
    else:
        print(f"❌ 获取用户信息失败: {response.text}")

def test_update_user_profile():
    """测试更新用户资料"""
    print_separator("测试：更新用户资料")
    
    request = user_pb2.UpdateProfileRequest(
        name="更新后的姓名",
        bio="这是更新后的个人简介",
        email="updated@example.com",
        wechat_id="updated_wechat",
        setting_privacy_course="private",
        setting_notification_switch=False
    )
    
    headers = get_auth_headers()
    headers['Content-Type'] = 'application/x-protobuf'
    
    response = requests.put(
        f"{BASE_URL}/api/v1/users/me",
        headers=headers,
        data=request.SerializeToString()
    )
    
    print(f"状态码: {response.status_code}")
    
    if response.status_code == 200:
        result = user_pb2.UpdateProfileResponse()
        result.ParseFromString(response.content)
        
        print(f"响应码: {result.code}")
        print(f"消息: {result.message}")
        print("✅ 用户资料更新成功")
    else:
        print(f"❌ 更新用户资料失败: {response.text}")

def test_change_password():
    """测试修改密码"""
    print_separator("测试：修改密码")
    
    request = user_pb2.ChangePasswordRequest(
        old_password="password123",
        new_password="newpassword123"
    )
    
    headers = get_auth_headers()
    headers['Content-Type'] = 'application/x-protobuf'
    
    response = requests.put(
        f"{BASE_URL}/api/v1/auth/password",
        headers=headers,
        data=request.SerializeToString()
    )
    
    print(f"状态码: {response.status_code}")
    
    if response.status_code == 200:
        result = user_pb2.ChangePasswordResponse()
        result.ParseFromString(response.content)
        
        print(f"响应码: {result.code}")
        print(f"消息: {result.message}")
        print("✅ 密码修改成功")
        
        # 重置密码回原来的值以便后续测试
        reset_request = user_pb2.ChangePasswordRequest(
            old_password="newpassword123",
            new_password="password123"
        )
        
        reset_response = requests.put(
            f"{BASE_URL}/api/v1/auth/password",
            headers=headers,
            data=reset_request.SerializeToString()
        )
        
        if reset_response.status_code == 200:
            print("✅ 密码已重置回原值")
        else:
            print("⚠️ 密码重置失败，可能影响后续测试")
    else:
        print(f"❌ 修改密码失败: {response.text}")

def test_user_logout():
    """测试用户退出"""
    print_separator("测试：用户退出")
    
    headers = get_auth_headers()
    headers['Content-Type'] = 'application/x-protobuf'
    
    response = requests.post(
        f"{BASE_URL}/api/v1/auth/logout",
        headers=headers
    )
    
    print(f"状态码: {response.status_code}")
    
    if response.status_code == 200:
        result = user_pb2.LogoutResponse()
        result.ParseFromString(response.content)
        
        print(f"响应码: {result.code}")
        print(f"消息: {result.message}")
        print("✅ 用户退出成功")
    else:
        print(f"❌ 用户退出失败: {response.text}")

def test_invalid_login():
    """测试无效登录"""
    print_separator("测试：无效登录（错误密码）")
    
    request = user_pb2.LoginRequest(
        student_id="2021001001",
        password="wrongpassword"
    )
    
    response = requests.post(
        f"{BASE_URL}/api/v1/auth/login",
        headers={'Content-Type': 'application/x-protobuf', 'Accept': 'application/x-protobuf'},
        data=request.SerializeToString()
    )
    
    print(f"状态码: {response.status_code}")
    
    if response.status_code == 401 or response.status_code == 400:
        print("✅ 正确：无效登录被拒绝")
        try:
            result = user_pb2.LoginResponse()
            result.ParseFromString(response.content)
            print(f"错误信息: {result.message}")
        except:
            print(f"错误信息: {response.text}")
    else:
        print("❌ 错误：无效登录应该被拒绝")

def run_user_tests():
    """运行用户模块所有测试"""
    print_separator("开始用户模块测试")
    
    # 1. 测试无效登录
    test_invalid_login()
    
    # 2. 测试用户登录
    test_user_login()
    
    # 3. 测试获取用户信息
    test_get_user_info()
    
    # 4. 测试更新用户资料
    test_update_user_profile()
    
    # 5. 再次获取用户信息验证更新
    test_get_user_info()
    
    # 6. 测试修改密码
    test_change_password()
    
    # 7. 测试用户注册
    test_user_register()
    
    # 8. 测试用户退出
    test_user_logout()

def run_course_tests():
    """运行课表模块所有测试"""
    print_separator("开始课表模块测试")
    
    # 0. 测试无认证访问
    test_unauthorized_access()
    
    # 1. 获取学期列表（公开接口）
    test_get_semesters()
    
    # 2. 获取全校课程（公开接口）
    test_get_public_courses(page=1, page_size=3)
    
    # 3. 获取用户课表（需要认证）
    test_get_schedule(semester_id=1)
    
    # 4. 添加课表项（需要认证）
    test_add_schedule_items(semester_id=1)

    # 4.1 再次获取课表（需要认证）
    test_get_schedule(semester_id=1)
    
    # 5. 更新课表项（需要认证，需要先有数据）
    test_update_schedule_item(item_id=2)
    
    # 6. 删除课表项（需要认证，需要先有数据）
    test_delete_schedule_item(item_id=2)

def main():
    print("\n" + "🎓 API 测试工具（支持模块化测试和JWT认证）".center(70))
    
    # 检查命令行参数
    if len(sys.argv) > 1:
        if sys.argv[1] in ['-h', '--help', 'help']:
            print("\n使用方法:")
            print("  python test_api.py [module] [user_id]")
            print("\n参数:")
            print("  module     测试模块：all(默认), course, user")
            print("  user_id    测试用户的 ID（默认: 1）")
            print("\n示例:")
            print("  python test_api.py                # 测试所有模块，用户ID 1")
            print("  python test_api.py course         # 只测试课表模块，用户ID 1")
            print("  python test_api.py user           # 只测试用户模块，用户ID 1")
            print("  python test_api.py course 2       # 测试课表模块，用户ID 2")
            print("  python test_api.py user 999       # 测试用户模块，用户ID 999")
            print("  python test_api.py all 3          # 测试所有模块，用户ID 3")
            return
        
        # 解析模块参数
        module = sys.argv[1].lower()
        if module not in ['all', 'course', 'user']:
            # 如果第一个参数不是模块名，尝试作为用户ID解析
            try:
                user_id = sys.argv[1]
                set_test_user(user_id)
                set_test_module("all")
            except:
                print(f"❌ 无效的模块或用户ID: {sys.argv[1]}")
                print("支持的模块: all, course, user")
                print("运行 'python test_api.py --help' 查看帮助")
                return
        else:
            set_test_module(module)
            
            # 解析用户ID参数
            if len(sys.argv) > 2:
                try:
                    user_id = sys.argv[2]
                    set_test_user(user_id)
                except:
                    print(f"❌ 无效的用户 ID: {sys.argv[2]}")
                    print("运行 'python test_api.py --help' 查看帮助")
                    return
    
    try:
        # 根据模块运行相应测试
        if TEST_MODULE == "course":
            run_course_tests()
        elif TEST_MODULE == "user":
            run_user_tests()
        else:  # all
            run_course_tests()
            print("\n" + "="*70)
            run_user_tests()
        
        print_separator("测试完成")
        print(f"✅ 测试模块: {TEST_MODULE}")
        print(f"✅ 使用的测试用户 ID: {TEST_USER_ID}")
        print(f"🔑 JWT Token: {JWT_TOKEN[:50]}..." if JWT_TOKEN else "❌ 未生成 Token")
        
    except requests.exceptions.ConnectionError:
        print("\n❌ 错误：无法连接到服务器")
        print("请确保服务器正在运行：cargo run")
    except Exception as e:
        print(f"\n❌ 错误：{e}")
        import traceback
        traceback.print_exc()

if __name__ == "__main__":
    main()
