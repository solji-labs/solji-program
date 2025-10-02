import { getUserKeypairs, getAllUserKeypairs, printAllUserAddresses } from "./user-generate";

describe("User Generate Test", () => {
    it("should generate fixed keypairs", () => {
        console.log("Testing user keypair generation...");
        
        // 测试单个用户获取
        const user1 = getUserKeypairs(0);
        const user2 = getUserKeypairs(1);
        
        console.log("User 1:", user1.publicKey.toString());
        console.log("User 2:", user2.publicKey.toString());
        
        // 验证每次调用返回相同的keypair
        const user1Again = getUserKeypairs(0);
        console.log("User 1 again:", user1Again.publicKey.toString());
        console.log("Same keypair:", user1.publicKey.equals(user1Again.publicKey));
        
        // 打印所有用户地址
        printAllUserAddresses();
        
        // 测试获取所有用户
        const allUsers = getAllUserKeypairs();
        console.log(`\nTotal users generated: ${allUsers.length}`);
        
        // 验证边界条件
        try {
            getUserKeypairs(-1);
        } catch (error) {
            console.log("✅ Correctly caught invalid index error:", (error as Error).message);
        }
        
        try {
            getUserKeypairs(10);
        } catch (error) {
            console.log("✅ Correctly caught out of bounds error:", (error as Error).message);
        }
    });
});
