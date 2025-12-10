#!/usr/bin/env python3
"""
简单的 JWT token 生成工具
用于测试课程模块的认证功能
"""

import json
import base64
import hmac
import hashlib
import time
from datetime import datetime, timedelta

def base64url_encode(data):
    """Base64URL 编码"""
    return base64.urlsafe_b64encode(data).decode('utf-8').rstrip('=')

def generate_jwt_token(user_id, secret="your_secret_key_here", expiration_hours=1):
    """生成 JWT token"""
    
    # Header
    header = {
        "alg": "HS256",
        "typ": "JWT"
    }
    
    # Payload
    now = int(time.time())
    payload = {
        "user_id": user_id,
        "iat": now,
        "exp": now + (expiration_hours * 3600)
    }
    
    # 编码 header 和 payload
    encoded_header = base64url_encode(json.dumps(header, separators=(',', ':')).encode())
    encoded_payload = base64url_encode(json.dumps(payload, separators=(',', ':')).encode())
    
    # 创建签名
    message = f"{encoded_header}.{encoded_payload}"
    signature = hmac.new(
        secret.encode(),
        message.encode(),
        hashlib.sha256
    ).digest()
    encoded_signature = base64url_encode(signature)
    
    # 组合 token
    token = f"{message}.{encoded_signature}"
    
    return token

def main():
    """主函数"""
    print("=== JWT Token 生成工具 ===")
    print()
    
    # 常用测试用户
    test_users = [
        (1, "admin"),
        (2, "user1"),
        (3, "user2"),
        (999, "test")
    ]
    
    secret = "your_secret_key_here"  # 与 .env 中的 JWT_SECRET 保持一致
    
    for user_id, username in test_users:
        token = generate_jwt_token(user_id, secret)
        exp_time = datetime.now() + timedelta(hours=1)
        
        print(f"用户: {username} (ID: {user_id})")
        print(f"Token: {token}")
        print(f"过期时间: {exp_time.strftime('%Y-%m-%d %H:%M:%S')}")
        print()
        
        # 生成 curl 测试命令
        print(f"测试命令:")
        print(f'curl -H "Authorization: Bearer {token}" http://localhost:3000/api/v1/schedule?semester_id=1')
        print("-" * 80)
        print()

if __name__ == "__main__":
    main()