import * as anchor from "@coral-xyz/anchor";
import { getTestContext } from "./utils/setup";
import { getUserKeypairs } from "./utils/user-generate";

/**
 * 抽签测试套件
 * 
 * 注意事项：
 * 1. 后端 draw_fortune 指令使用 Option<AccountInfo> 接收 randomness_account
 * 2. 在 localnet 环境下，不需要提供 randomness_account（使用伪随机数）
 * 3. 在 devnet/mainnet 环境下，应该提供有效的 Switchboard 随机数账户
 * 4. 如果未提供 randomness_account，后端会降级到伪随机数（slot + timestamp）
 */

// 运势类型映射
function getFortuneText(fortune: any): string {
    const fortuneMap: { [key: string]: string } = {
        'GreatLuck': '大吉',
        'Lucky': '吉',
        'Good': '小吉',
        'Normal': '正常',
        'Nobad': '小凶',
        'Bad': '凶',
        'VeryBad': '大凶'
    };
    
    // 如果fortune是对象，获取第一个键
    if (typeof fortune === 'object' && fortune !== null) {
        const key = Object.keys(fortune)[0];
        return fortuneMap[key] || '未知';
    }
    
    return fortuneMap[fortune] || '未知';
}

function getFortuneDescription(fortune: any): string {
    const descriptionMap: { [key: string]: string } = {
        'GreatLuck': '万事顺意，心想事成',
        'Lucky': '诸事顺利，渐入佳境',
        'Good': '平平淡淡，稳中求进',
        'Normal': '平平淡淡，顺其自然',
        'Nobad': '小心谨慎，化险为夷',
        'Bad': '诸事不利，谨慎为上',
        'VeryBad': '凶险重重，静待时机'
    };
    
    // 如果fortune是对象，获取第一个键
    if (typeof fortune === 'object' && fortune !== null) {
        const key = Object.keys(fortune)[0];
        return descriptionMap[key] || '运势未明，静观其变';
    }
    
    return descriptionMap[fortune] || '运势未明，静观其变';
}

describe("draw fortune", () => {

    const ctx = getTestContext();

    console.log("User Program Test Suite");
    console.log("========================");
    console.log("Program ID: ", ctx.program.programId.toString());

    it("should draw fortune", async () => {
        // 生成新用户并进行airdrop
        let randomUserIndex = Math.floor(Math.random() * 8);
        const user = getUserKeypairs(randomUserIndex);
        console.log("User: ", user.publicKey.toString());

        // 检查用户SOL余额，如果不足则进行airdrop
        const balance = await ctx.provider.connection.getBalance(user.publicKey);
        console.log(`User balance: ${balance / 1e9} SOL`);
        if (balance < 1e9) { // 如果余额小于1 SOL
            console.log("Insufficient balance, airdropping...");
            await ctx.airdropToUser(user.publicKey);
        }

        // 检查用户状态是否已存在，如果不存在则初始化
        const userStatePda = ctx.getUserStatePda(user.publicKey);
        try {
            await ctx.program.account.userState.fetch(userStatePda);
            console.log("User state already exists, skipping initialization...");
        } catch (error) {
            console.log("Initializing user state...");
            await ctx.initUser(user);
            console.log("User state initialized successfully!");
        }
 

        const { tx, fortuneResult } = await ctx.drawFortune(user);

        console.log(`\n🎊 抽签完成！交易签名: ${tx}`);
        
        if (fortuneResult) {
            console.log("\n✨ 事件监听成功获取到抽签结果！");
        } else {
            console.log("\n⚠️  未能通过事件获取抽签结果，可能需要调整等待时间");
        }
        
        console.log("\n📊 查看用户状态和寺庙状态的变化：");

        ctx.printUserState(userStatePda);
        ctx.printTempleConfig();


    });
});


