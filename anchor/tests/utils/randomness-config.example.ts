import { PublicKey } from "@solana/web3.js";

/**
 * Switchboard 随机数账户配置示例
 * 
 * 使用说明：
 * 1. 复制此文件为 randomness-config.ts
 * 2. 根据你的网络环境配置相应的随机数账户地址
 * 3. 在 setup.ts 中导入并使用
 * 
 * 获取 Switchboard 随机数账户的方法：
 * - Devnet: 使用 Switchboard On-Demand 服务创建
 * - Mainnet: 使用 Switchboard On-Demand 服务创建
 * - 文档: https://docs.switchboard.xyz/
 */

export interface RandomnessConfig {
    // Devnet 随机数账户
    devnet?: PublicKey;
    // Mainnet 随机数账户
    mainnet?: PublicKey;
}

/**
 * 随机数账户配置
 * 
 * 注意：这些是示例地址，实际使用时需要替换为你自己的 Switchboard 账户
 */
export const RANDOMNESS_ACCOUNTS: RandomnessConfig = {
    // Devnet 示例地址（需要替换为实际的 Switchboard 账户）
    devnet: new PublicKey('GvDMxPzN1sCj7L26YDK2HnMRXEQmQ2aemov8YBtPS7vR'),
    
    // Mainnet 示例地址（需要替换为实际的 Switchboard 账户）
    // mainnet: new PublicKey('YOUR_MAINNET_RANDOMNESS_ACCOUNT'),
};

/**
 * 根据网络环境获取随机数账户
 * 
 * @param network - 网络 URL 或网络名称
 * @returns Switchboard 随机数账户的公钥，如果未配置则返回 undefined
 */
export function getRandomnessAccount(network: string): PublicKey | undefined {
    // 判断网络类型
    if (network.includes('localhost') || network.includes('127.0.0.1')) {
        // Localnet 不需要随机数账户
        return undefined;
    } else if (network.includes('devnet')) {
        return RANDOMNESS_ACCOUNTS.devnet;
    } else if (network.includes('mainnet')) {
        return RANDOMNESS_ACCOUNTS.mainnet;
    }
    
    // 默认返回 undefined，使用后端降级方案
    return undefined;
}

/**
 * 检查是否需要随机数账户
 * 
 * @param network - 网络 URL 或网络名称
 * @returns 是否需要随机数账户
 */
export function needsRandomnessAccount(network: string): boolean {
    return !network.includes('localhost') && !network.includes('127.0.0.1');
}
