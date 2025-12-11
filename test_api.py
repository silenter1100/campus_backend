#!/usr/bin/env python3
"""
课表模块 API 测试脚本（支持 JWT 认证）
使用方法：
1. 安装依赖：pip install requests protobuf
2. 编译 proto：protoc --python_out=. proto/course.proto
3. 运行测试：python test_api.py
"""

import requests
import sys
import os

# 添加生成的 proto 文件路径
sys.path.insert(0, os.path.dirname(__file__))

try:
    from proto import course_pb2
    from generate_token import generate_jwt_token
except ImportError as e:
    print(f"错误：找不到必要模块: {e}")
    print("请确保：")
    print("1. 运行：protoc --python_out=. proto/course.proto")
    print("2. generate_token.py 文件存在")
    sys.exit(1)

BASE_URL = "http://localhost:3000"

# JWT 认证配置
TEST_USER_ID = 1  # 测试用户 ID，可以通过环境变量或命令行参数修改
JWT_TOKEN = None  # 全局 token 变量

def set_test_user(user_id):
    """切换测试用户"""
    global TEST_USER_ID, JWT_TOKEN
    TEST_USER_ID = user_id
    JWT_TOKEN = None  # 重置 token，下次请求时会重新生成
    print(f"🔄 切换到用户 ID: {user_id}")

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

def main():
    print("\n" + "🎓 课表模块 API 测试工具（JWT 认证版）".center(60))
    
    # 检查命令行参数
    if len(sys.argv) > 1:
        if sys.argv[1] in ['-h', '--help', 'help']:
            print("\n使用方法:")
            print("  python test_api.py [user_id]")
            print("\n参数:")
            print("  user_id    测试用户的 ID（默认: 1）")
            print("\n示例:")
            print("  python test_api.py        # 使用默认用户 ID 1")
            print("  python test_api.py 2      # 使用用户 ID 2")
            print("  python test_api.py 999    # 使用用户 ID 999")
            return
        
        try:
            user_id = int(sys.argv[1])
            set_test_user(user_id)
        except ValueError:
            print(f"❌ 无效的用户 ID: {sys.argv[1]}")
            print("使用方法: python test_api.py [user_id]")
            print("运行 'python test_api.py --help' 查看帮助")
            return
    
    try:
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
        
        print_separator("测试完成")
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
