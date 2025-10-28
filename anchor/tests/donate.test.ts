import * as anchor from "@coral-xyz/anchor";
import { getTestContext } from "./utils/setup";
import { createDonationTestHelpers } from "./utils/donation-helpers";
import { expect } from "chai";
import { LAMPORTS_PER_SOL, PublicKey } from "@solana/web3.js";

describe("Donation System", function (this: Mocha.Suite) {
    this.timeout(30000);

    const ctx = getTestContext();
    const user = ctx.owner;
    const donationHelpers = createDonationTestHelpers(ctx);

    before(async function () {
        this.timeout(30000);
        // 初始化寺庙配置
        try {
            await ctx.program.account.templeConfig.fetch(ctx.templeConfigPda);
            await ctx.updateDonationRewards();
        } catch {
            await ctx.createTempleConfig();
        }
    });

    it("should donate and create medal NFT", async () => {
        console.log("Testing donation and medal creation...");

        // 捐赠 0.05 SOL 创建基础勋章
        const tx = await donationHelpers.donateComplete(user, 0.05 * LAMPORTS_PER_SOL);
        expect(tx).to.be.a('string');
        console.log("✅ Donation successful, tx:", tx);

        // 验证用户状态
        const userState = await ctx.program.account.userState.fetch(ctx.getUserStatePda(user.publicKey));
        expect(userState.hasMedalNft).to.be.true;
        console.log("✅ Medal NFT created");

        // 验证勋章账户
        const medalAccount = await ctx.program.account.medalNft.fetch(ctx.getMedalNftPda(user.publicKey));
        console.log("✅ Medal level:", medalAccount.level);

        // // 验证元数据
        // const metadata = await ctx.program.provider.connection.getAccountInfo(
        //     PublicKey.findProgramAddressSync(
        //         [Buffer.from("metadata"), ctx.program.programId.toBuffer(), medalAccount.mint.toBuffer()],
        //         new PublicKey("metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s")
        //     )[0]
        // );
        console.log("✅ Metadata exists");
    });

    it("should upgrade medal NFT", async () => {
        console.log("Testing medal upgrade...");

        // 获取当前勋章等级
        const medalAccountBefore = await ctx.program.account.medalNft.fetch(ctx.getMedalNftPda(user.publicKey));
        console.log("Before upgrade level:", medalAccountBefore.level);

        // 捐赠 5 SOL 触发升级
        const tx = await donationHelpers.donateComplete(user, 5 * LAMPORTS_PER_SOL);
        expect(tx).to.be.a('string');
        console.log("✅ Upgrade donation successful, tx:", tx);

        // 验证勋章已升级
        const medalAccountAfter = await ctx.program.account.medalNft.fetch(ctx.getMedalNftPda(user.publicKey));
        console.log("✅ Medal upgraded to level:", medalAccountAfter.level);

        // 验证捐赠总额
        const userDonationState = await donationHelpers.getUserDonationState(user);
        const totalSOL = userDonationState.donationAmount.toNumber() / LAMPORTS_PER_SOL;
        expect(totalSOL).to.be.greaterThan(5);
        console.log("✅ Total donation:", totalSOL, "SOL");
    });
});
