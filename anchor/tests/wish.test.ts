import * as anchor from "@coral-xyz/anchor";
import { getTestContext } from "./utils/setup";
import { getUserKeypairs } from "./utils/user-generate";
import * as crypto from "crypto";

describe("wish", () => {

    const ctx = getTestContext();

    console.log("Creator Program Test Suite");
    console.log("========================");
    console.log("Program ID: ", ctx.program.programId.toString());

    it("should wish", async () => {
        // // 生成新用户并进行airdrop
        // let randomUserIndex = Math.floor(Math.random() * 7)+1;
        // const creator = getUserKeypairs(randomUserIndex);
        // const liker = getUserKeypairs(randomUserIndex + 1);
        // console.log("Creator: ", creator.publicKey.toString());
        // console.log("Liker: ", liker.publicKey.toString());

        // // 检查用户SOL余额，如果不足则进行airdrop
        // const balance = await ctx.provider.connection.getBalance(creator.publicKey);
        // const likerBalance = await ctx.provider.connection.getBalance(liker.publicKey);
        // console.log(`Creator balance: ${balance / 1e9} SOL`);
        // console.log(`Liker balance: ${likerBalance / 1e9} SOL`);
        // if (balance < 1e9) { // 如果余额小于1 SOL
        //     console.log("Insufficient balance, airdropping...");
        //     await ctx.airdropToUser(creator.publicKey);
        // }

        // if (likerBalance < 1e9) { // 如果余额小于1 SOL
        //     console.log("Insufficient balance, airdropping...");
        //     await ctx.airdropToUser(liker.publicKey);
        // }


        // let creator = await ctx.getUserKeypairsAndTransferSOL(0.01);
        // let liker = await ctx.getUserKeypairsAndTransferSOL(0.01);

        let creator = ctx.authority;


        // 检查用户状态是否已存在，如果不存在则初始化
        const userStatePda = ctx.getUserStatePda(creator.publicKey);
        try {
            await ctx.program.account.userState.fetch(userStatePda);
            console.log("User state already exists, skipping initialization...");
        } catch (error) {
            console.log("Initializing user state...");
            await ctx.initUser(creator);
            console.log("User state initialized successfully!");
        }

        const wishId = Date.now();
        const contentHash = randomWishContent();
        const isAnonymous = randomBoolean();
        let tx = await ctx.createWish(creator, wishId, contentHash, isAnonymous);


          let wishPda = ctx.getWishPda(creator.publicKey, wishId);
                let wishAccount = await ctx.program.account.wish.fetch(wishPda);
                console.log("Wish account after like: ", wishAccount);


        // if (tx) {
        //     console.log("Wish created successfully!");
        //     let likeTx = await ctx.likeWish(liker, creator.publicKey, wishId);

        //     if (likeTx) {
        //         console.log("Wish liked successfully!");
        //         let likePda = ctx.getWishLikePda(liker.publicKey, creator.publicKey, wishId);
        //         let likeAccount = await ctx.program.account.wishLike.fetch(likePda);
        //         console.log("Like account: ", likeAccount);

        //         let wishPda = ctx.getWishPda(creator.publicKey, wishId);
        //         let wishAccount = await ctx.program.account.wish.fetch(wishPda);
        //         console.log("Wish account after like: ", wishAccount);
 

        //         let cancelLikeTx = await ctx.cancelLikeWish(liker, creator.publicKey, wishId);

        //         if (cancelLikeTx) {
 

        //             let wishAccountAfterCancel = await ctx.program.account.wish.fetch(wishPda);
        //             if (wishAccountAfterCancel) {
        //                 console.log("Wish account after cancel: ", wishAccountAfterCancel);
        //             }
        //         }
        //     }

        //     // ctx.printUserState(userStatePda);
        //     // ctx.printTempleState();
        // }
    });
});


function randomBoolean(): boolean {
    // return Math.random() >= 0.5;
    return false;
}





function randomWishContent(): number[] {
    const content: WishContent = {
        text: `Random wish #${Math.floor(Math.random() * 10000)}`,
        style: getRandomStyle(),
        images: generateRandomImages(),
        tags: generateRandomTags(),
        timestamp: Date.now()
    };

    // 将 WishContent 对象序列化为 JSON 字符串
    const contentString = JSON.stringify(content);

    // 使用 SHA-256 生成哈希
    const hash = crypto.createHash('sha256')
        .update(contentString)
        .digest();

    // 转换为 number 数组 (32 bytes)
    return Array.from(hash);
}

function getRandomStyle(): string {
    const styles = ['cz', 'justin_sun', 'default'];
    return styles[Math.floor(Math.random() * styles.length)];
}

function generateRandomImages(): string[] {
    const count = Math.floor(Math.random() * 3) + 1; // 1-3张图片
    const images: string[] = [];
    for (let i = 0; i < count; i++) {
        images.push(`https://example.com/wish-image-${Math.floor(Math.random() * 1000)}.jpg`);
    }
    return images;
}

function generateRandomTags(): string[] {
    const allTags = ['wish', 'hope', 'dream', 'blessing', 'fortune', 'health', 'love', 'success'];
    const count = Math.floor(Math.random() * 3) + 1; // 1-3个标签
    const tags: string[] = [];

    for (let i = 0; i < count; i++) {
        const randomTag = allTags[Math.floor(Math.random() * allTags.length)];
        if (!tags.includes(randomTag)) {
            tags.push(randomTag);
        }
    }

    return tags;
}


/**
 * 愿望内容结构
 */
export interface WishContent {
    text: string;
    style?: string; // 绘马风格：'cz', 'justin_sun', 'default'
    images?: string[]; // 图片URL数组
    tags?: string[]; // 标签
    timestamp: number;
}