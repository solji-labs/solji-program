import { Keypair } from "@solana/web3.js";
import * as crypto from "crypto";

const USER_COUNT = 1000;
const USER_KEYPAIRS: Keypair[] = [];

/**
 * 获取固定的测试用户Keypair
 * @param index 用户索引 (0-9)
 * @returns Keypair 对象
 */
export function getUserKeypairs(index: number): Keypair {
    if (index < 0 || index >= USER_COUNT) {
        throw new Error(`Invalid user index: ${index}. Must be between 0 and ${USER_COUNT - 1}`);
    }
    
    // 懒加载：只在第一次调用时生成所有Keypair
    if (USER_KEYPAIRS.length === 0) {
        initializeUserKeypairs();
    }
    
    return USER_KEYPAIRS[index];
}

/**
 * 获取所有测试用户的Keypair
 * @returns Keypair数组
 */
export function getAllUserKeypairs(): Keypair[] {
    if (USER_KEYPAIRS.length === 0) {
        initializeUserKeypairs();
    }
    return [...USER_KEYPAIRS]; // 返回副本，防止外部修改
}

/**
 * 初始化所有用户Keypair
 */
function initializeUserKeypairs(): void {
    for (let i = 0; i < USER_COUNT; i++) {
        const seed = generateSeed(`test_user_${i + 1}`);
        USER_KEYPAIRS.push(Keypair.fromSeed(seed));
    }
}

/**
 * 从字符串生成32字节的种子
 * @param seedString 种子字符串
 * @returns 32字节的Uint8Array
 */
function generateSeed(seedString: string): Uint8Array {
    // 使用SHA-256生成固定的32字节种子
    const hash = crypto.createHash('sha256').update(seedString).digest();
    return new Uint8Array(hash);
}

/**
 * 打印所有测试用户的公钥地址（用于调试）
 */
export function printAllUserAddresses(): void {
    console.log("\n🔑 Test User Addresses:");
    console.log("========================");
    for (let i = 0; i < USER_COUNT; i++) {
        const keypair = getUserKeypairs(i);
        console.log(`User ${i + 1}: ${keypair.publicKey.toString()}`);
    }
}
