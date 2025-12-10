#!/usr/bin/env python3
"""
JWT 认证测试脚本
"""

import requests
import json
from generate_token import generate_jwt_token

def test_jwt_auth():
    """测试 JWT 认证"""
    base_url = "http://localhost:3000"
    
    # 生成测试 token
    user_id = 1
    token = generate_jwt_token(user_id)
    
    print(f"测试用户ID: {user_id}")
    print(f"生成的Token: {token}")
    print()
    
    # 测试需要认证的接口
    headers = {
        "Authorization": f"Bearer {token}",
        "Content-Type": "application/json"
    }
    
    # 测试获取课表
    print("=== 测试获取课表 ===")
    try:
        response = requests.get(
            f"{base_url}/api/v1/schedule",
            params={"semester_id": 1},
            headers=headers,
            timeout=5
        )
        print(f"状态码: {response.status_code}")
        print(f"响应头: {dict(response.headers)}")
        if response.content:
            print(f"响应长度: {len(response.content)} bytes")
        else:
            print("响应为空")
    except requests.exceptions.RequestException as e:
        print(f"请求失败: {e}")
    
    print()
    
    # 测试不带 token 的请求
    print("=== 测试无认证请求 ===")
    try:
        response = requests.get(
            f"{base_url}/api/v1/schedule",
            params={"semester_id": 1},
            timeout=5
        )
        print(f"状态码: {response.status_code}")
        if response.content:
            print(f"响应: {response.content.decode('utf-8', errors='ignore')}")
    except requests.exceptions.RequestException as e:
        print(f"请求失败: {e}")
    
    print()
    
    # 测试公开接口（不需要认证）
    print("=== 测试公开接口 ===")
    try:
        response = requests.get(
            f"{base_url}/api/v1/semesters",
            timeout=5
        )
        print(f"状态码: {response.status_code}")
        if response.content:
            print(f"响应长度: {len(response.content)} bytes")
    except requests.exceptions.RequestException as e:
        print(f"请求失败: {e}")

if __name__ == "__main__":
    test_jwt_auth()