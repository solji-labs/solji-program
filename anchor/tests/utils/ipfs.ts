
// import { create as ipfsHttpClient } from 'ipfs-http-client';
// import { INFURA_PROJECT_ID, INFURA_PROJECT_SECRET } from '../config';

// // IPFS客户端配置
// const ipfs = ipfsHttpClient({
//     host: 'ipfs.infura.io',
//     port: 5001,
//     protocol: 'https',
//     headers: {
//         authorization: `Basic ${Buffer.from(
//             `${process.env.INFURA_PROJECT_ID}:${process.env.INFURA_PROJECT_SECRET}`
//         ).toString('base64')}`,
//     },
// });

// /**
//  * 愿望内容结构
//  */
// export interface WishContent {
//     text: string;
//     style?: string; // 绘马风格：'cz', 'justin_sun', 'default'
//     images?: string[]; // 图片URL数组
//     tags?: string[]; // 标签
//     timestamp: number;
// }

// /**
//  * 上传愿望内容到IPFS
//  * @param content - 愿望内容
//  * @returns IPFS哈希
//  */
// export async function uploadWishToIPFS(content: WishContent): Promise<string> {
//     try {
//         const contentJson = JSON.stringify(content);
//         const result = await ipfs.add(contentJson);
//         console.log(`Content uploaded to IPFS: ${result.path}`);
//         return result.path;
//     } catch (error) {
//         console.error("Failed to upload to IPFS:", error);
//         throw error;
//     }
// }

// /**
//  * 从IPFS获取愿望内容
//  * @param hash - IPFS哈希
//  * @returns 愿望内容
//  */
// export async function getWishFromIPFS(hash: string): Promise<WishContent> {
//     try {
//         const chunks = [];
//         for await (const chunk of ipfs.cat(hash)) {
//             chunks.push(chunk);
//         }
//         const content = Buffer.concat(chunks).toString();
//         return JSON.parse(content);
//     } catch (error) {
//         console.error("Failed to fetch from IPFS:", error);
//         throw error;
//     }
// }

// /**
//  * 将IPFS哈希转换为32字节数组
//  * @param ipfsHash - IPFS哈希字符串
//  * @returns 32字节数组
//  */
// export function ipfsHashToBytes32(ipfsHash: string): number[] {
//     // 移除 'Qm' 前缀并转换为bytes32
//     const hashBytes = Buffer.from(ipfsHash.slice(2), 'base58');
//     const bytes32 = new Array(32).fill(0);
    
//     // 复制哈希字节到数组中
//     for (let i = 0; i < Math.min(hashBytes.length, 32); i++) {
//         bytes32[i] = hashBytes[i];
//     }
    
//     return bytes32;
// }