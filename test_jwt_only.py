#!/usr/bin/env python3
"""
仅测试 JWT Token 生成和解析
不需要启动服务器
"""

import sys
from generate_token import generate_jwt_token
import json
import base64

def decode_jwt_payload(token):
    """解码 JWT payload（不验证签名）"""
    try:
        # JWT 格式: header.payload.signature
        parts = token.split('.')
        if len(parts) != 3:
            return None
        
        # 解码 payload（第二部分）
        payload = parts[1]
        # 添加必要的填充
        payload += '=' * (4 - len(payload) % 4)
        decoded = base64.urlsafe_b64decode(payload)
        return json.loads(decoded)
    except Exception as e:
        print(f"解码失败: {e}")
        return None

def test_jwt_generation():
    """测试 JWT 生成和解析"""
    print("=== JWT Token 生成和解析测试 ===\n")
    
    test_users = [1, 2, 3, 999]
    
    for user_id in test_users:
        print(f"测试用户 ID: {user_id}")
        
        # 生成 token
        try:
            token = generate_jwt_token(user_id)
            print(f"✅ Token 生成成功")
            print(f"Token: {token}")
            
            # 解析 payload
            payload = decode_jwt_payload(token)
            if payload:
                print(f"✅ Token 解析成功")
                print(f"Payload: {json.dumps(payload, indent=2)}")
                
                # 验证用户 ID
                if payload.get('user_id') == user_id:
                    print(f"✅ 用户 ID 匹配: {user_id}")
                else:
                    print(f"❌ 用户 ID 不匹配: 期望 {user_id}, 实际 {payload.get('user_id')}")
            else:
                print(f"❌ Token 解析失败")
                
        except Exception as e:
            print(f"❌ Token 生成失败: {e}")
        
        print("-" * 60)
        print()

def main():
    if len(sys.argv) > 1 and sys.argv[1] in ['-h', '--help', 'help']:
        print("JWT Token 测试工具")
        print("使用方法: python test_jwt_only.py")
        return
    
    test_jwt_generation()
    
    print("🎯 测试完成！")
    print("\n下一步:")
    print("1. 启动服务器: cargo run")
    print("2. 运行完整测试: python test_api.py")

if __name__ == "__main__":
    main()