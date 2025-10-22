# 互动功能 API

## 1. 抽签 (draw_fortune)

### 功能描述
用户消耗功德值进行抽签，获得运势结果。每日首次抽签免费，后续每次消耗 5 功德值，每次抽签奖励 2 功德值。

### 参数
无

### 返回值

```typescript
interface DrawFortuneResult {
  reduceKarmaPoints: anchor.BN;  // 消耗的功德值
  rewardKarmaPoints: anchor.BN;  // 奖励的功德值
  currentTimestamp: number;      // 当前时间戳
  isFreeDraw: boolean;           // 是否为免费抽签
  fortune: FortuneResult;        // 运势结果
}

enum FortuneResult {
  GreatLuck,  // 大吉 (5%)
  Lucky,      // 吉 (10%)
  Good,       // 小吉 (20%)
  Normal,     // 正常 (30%)
  Nobad,      // 小凶 (20%)
  Bad,        // 凶 (10%)
  VeryBad,    // 大凶 (5%)
}
```

### 账户
| 账户名 | 类型 | 说明 |
|--------|------|------|
| userState | PDA (可写) | 用户状态 |
| user | Signer (可写) | 用户账户 |
| templeConfig | PDA (可写) | 寺庙配置 |
| randomnessAccount | AccountInfo | 随机数账户（生产环境） |
| systemProgram | Program | 系统程序 |

### TypeScript 示例

```typescript
async function drawFortune(
  program: anchor.Program,
  user: anchor.web3.Keypair
): Promise<DrawFortuneResult> {
  const [userStatePda] = PublicKey.findProgramAddressSync(
    [Buffer.from("user_state_v1"), user.publicKey.toBuffer()],
    program.programId
  );

  const [templeConfigPda] = PublicKey.findProgramAddressSync(
    [Buffer.from("temple_config_v1")],
    program.programId
  );

  // 调用指令
  const tx = await program.methods
    .drawFortune()
    .accounts({
      userState: userStatePda,
      user: user.publicKey,
      templeConfig: templeConfigPda,
      systemProgram: anchor.web3.SystemProgram.programId,
    })
    .signers([user])
    .rpc();

  console.log("✅ Fortune drawn:", tx);

  // 解析返回值（需要从交易日志中获取）
  // 实际应用中可以通过事件监听或解析日志获取
  return {
    reduceKarmaPoints: new anchor.BN(0), // 首次免费
    rewardKarmaPoints: new anchor.BN(2),
    currentTimestamp: Date.now() / 1000,
    isFreeDraw: true,
    fortune: FortuneResult.Good,
  };
}

// 使用示例
const result = await drawFortune(program, userKeypair);
console.log("运势:", getFortuneText(result.fortune));
console.log("是否免费:", result.isFreeDraw);
```

### 运势说明

```typescript
function getFortuneText(fortune: FortuneResult): string {
  const fortuneMap = {
    [FortuneResult.GreatLuck]: "大吉 - 万事顺意，心想事成",
    [FortuneResult.Lucky]: "吉 - 诸事顺利，渐入佳境",
    [FortuneResult.Good]: "小吉 - 平平淡淡，稳中求进",
    [FortuneResult.Normal]: "正常 - 平平淡淡，顺其自然",
    [FortuneResult.Nobad]: "小凶 - 小心谨慎，化险为夷",
    [FortuneResult.Bad]: "凶 - 诸事不利，谨慎为上",
    [FortuneResult.VeryBad]: "大凶 - 凶险重重，静待时机",
  };
  return fortuneMap[fortune];
}
```

### 概率分布

| 运势 | 概率 | 说明 |
|------|------|------|
| 大吉 | 5% | 极好运势 |
| 吉 | 10% | 好运势 |
| 小吉 | 20% | 较好运势 |
| 正常 | 30% | 普通运势 |
| 小凶 | 20% | 较差运势 |
| 凶 | 10% | 差运势 |
| 大凶 | 5% | 极差运势 |

### 命令行示例

```bash
# 使用 Anchor CLI
anchor run draw-fortune

# 或使用测试脚本
ANCHOR_PROVIDER_URL=https://api.devnet.solana.com \
ANCHOR_WALLET=~/.config/solana/id.json \
npx mocha --require tsx tests/draw-fortune.test.ts --timeout 60000
```

### 功德值消耗规则

- **首次抽签**: 免费（每日首次）
- **后续抽签**: 消耗 5 功德值
- **奖励**: 每次抽签奖励 2 功德值
- **净消耗**: 3 功德值/次（非首次）

---

## 2. 许愿 (create_wish)

### 功能描述
用户创建许愿，消耗功德值。每日前 3 次许愿免费，后续每次消耗 10 功德值。许愿有 10% 概率获得御守铸造机会。

### 参数
- `wishId`: anchor.BN - 许愿ID（建议使用时间戳）
- `contentHash`: [u8; 32] - 许愿内容哈希（32字节）
- `isAnonymous`: boolean - 是否匿名许愿

### 返回值

```typescript
interface CreateWishResult {
  wishId: anchor.BN;             // 许愿ID
  contentHash: number[];         // 内容哈希
  isAnonymous: boolean;          // 是否匿名
  isFreewish: boolean;           // 是否免费许愿
  isAmuletDropped: boolean;      // 是否掉落御守
  rewardKarmaPoints: anchor.BN;  // 奖励功德值
  reduceKarmaPoints: anchor.BN;  // 消耗功德值
  currentTimestamp: number;      // 当前时间戳
}
```

### 账户
| 账户名 | 类型 | 说明 |
|--------|------|------|
| wish | PDA (可写, 初始化) | 许愿账户 |
| userState | PDA (可写) | 用户状态 |
| user | Signer (可写) | 用户账户 |
| templeConfig | PDA (可写) | 寺庙配置 |
| systemProgram | Program | 系统程序 |

### TypeScript 示例

```typescript
import { createHash } from "crypto";

async function createWish(
  program: anchor.Program,
  user: anchor.web3.Keypair,
  content: string,
  isAnonymous: boolean
): Promise<CreateWishResult> {
  // 生成许愿ID（使用时间戳）
  const wishId = new anchor.BN(Date.now());

  // 计算内容哈希
  const contentHash = Array.from(
    createHash("sha256").update(content).digest()
  );

  // 计算 PDA
  const [wishPda] = PublicKey.findProgramAddressSync(
    [
      Buffer.from("wish_v1"),
      user.publicKey.toBuffer(),
      wishId.toArrayLike(Buffer, "le", 8)
    ],
    program.programId
  );

  const [userStatePda] = PublicKey.findProgramAddressSync(
    [Buffer.from("user_state_v1"), user.publicKey.toBuffer()],
    program.programId
  );

  const [templeConfigPda] = PublicKey.findProgramAddressSync(
    [Buffer.from("temple_config_v1")],
    program.programId
  );

  // 调用指令
  const tx = await program.methods
    .createWish(wishId, contentHash, isAnonymous)
    .accounts({
      wish: wishPda,
      userState: userStatePda,
      user: user.publicKey,
      templeConfig: templeConfigPda,
      systemProgram: anchor.web3.SystemProgram.programId,
    })
    .signers([user])
    .rpc();

  console.log("✅ Wish created:", tx);

  // 返回结果（实际应从交易日志解析）
  return {
    wishId,
    contentHash,
    isAnonymous,
    isFreewish: true,
    isAmuletDropped: false,
    rewardKarmaPoints: new anchor.BN(1),
    reduceKarmaPoints: new anchor.BN(0),
    currentTimestamp: Date.now() / 1000,
  };
}

// 使用示例
const result = await createWish(
  program,
  userKeypair,
  "愿家人平安健康",
  false // 不匿名
);

console.log("许愿ID:", result.wishId.toString());
console.log("是否免费:", result.isFreewish);
console.log("是否获得御守:", result.isAmuletDropped);
```

### 命令行示例

```bash
# 使用 Anchor CLI
anchor run wish

# 或使用测试脚本
ANCHOR_PROVIDER_URL=https://api.devnet.solana.com \
ANCHOR_WALLET=~/.config/solana/id.json \
npx mocha --require tsx tests/wish.test.ts --timeout 60000
```

### 查询许愿

```typescript
async function getWish(
  program: anchor.Program,
  creator: PublicKey,
  wishId: anchor.BN
) {
  const [wishPda] = PublicKey.findProgramAddressSync(
    [
      Buffer.from("wish_v1"),
      creator.toBuffer(),
      wishId.toArrayLike(Buffer, "le", 8)
    ],
    program.programId
  );

  const wish = await program.account.wish.fetch(wishPda);

  console.log("Creator:", wish.creator.toString());
  console.log("Wish ID:", wish.wishId.toString());
  console.log("Content Hash:", wish.contentHash);
  console.log("Is Anonymous:", wish.isAnonymous);
  console.log("Like Count:", wish.likeCount);
  console.log("Is Amulet Dropped:", wish.isAmuletDropped);
  console.log("Created At:", new Date(wish.createdAt.toNumber() * 1000));

  return wish;
}
```

### 功德值消耗规则

- **前 3 次**: 免费（每日）
- **后续**: 消耗 10 功德值
- **奖励**: 每次许愿奖励 1 功德值
- **净消耗**: 9 功德值/次（非免费）

### 御守掉落

- **掉落概率**: 10%
- **用途**: 可用于铸造特殊御守 NFT（未来功能）
- **记录**: 记录在许愿账户的 `isAmuletDropped` 字段

---

## 3. 点赞许愿 (like_wish)

### 功能描述
用户为其他用户的许愿点赞，增加许愿的点赞数。每个用户对同一许愿只能点赞一次。

### 参数
- `wishId`: anchor.BN - 许愿ID

### 账户
| 账户名 | 类型 | 说明 |
|--------|------|------|
| wishLike | PDA (可写, 初始化) | 点赞账户 |
| wish | PDA (可写) | 许愿账户 |
| creator | AccountInfo (可写) | 许愿创建者 |
| liker | Signer (可写) | 点赞者 |
| systemProgram | Program | 系统程序 |

### TypeScript 示例

```typescript
async function likeWish(
  program: anchor.Program,
  liker: anchor.web3.Keypair,
  creator: PublicKey,
  wishId: anchor.BN
): Promise<string> {
  // 计算 PDA
  const [wishLikePda] = PublicKey.findProgramAddressSync(
    [
      Buffer.from("wish_like_v1"),
      liker.publicKey.toBuffer(),
      creator.toBuffer(),
      wishId.toArrayLike(Buffer, "le", 8)
    ],
    program.programId
  );

  const [wishPda] = PublicKey.findProgramAddressSync(
    [
      Buffer.from("wish_v1"),
      creator.toBuffer(),
      wishId.toArrayLike(Buffer, "le", 8)
    ],
    program.programId
  );

  // 调用指令
  const tx = await program.methods
    .likeWish(wishId)
    .accounts({
      wishLike: wishLikePda,
      wish: wishPda,
      creator: creator,
      liker: liker.publicKey,
      systemProgram: anchor.web3.SystemProgram.programId,
    })
    .signers([liker])
    .rpc();

  console.log("✅ Wish liked:", tx);
  return tx;
}

// 使用示例
const tx = await likeWish(
  program,
  likerKeypair,
  creatorPublicKey,
  wishId
);
```

### 命令行示例

```bash
ANCHOR_PROVIDER_URL=https://api.devnet.solana.com \
ANCHOR_WALLET=~/.config/solana/id.json \
npx mocha --require tsx tests/wish.test.ts --timeout 60000
```

### 查询点赞状态

```typescript
async function checkIfLiked(
  program: anchor.Program,
  liker: PublicKey,
  creator: PublicKey,
  wishId: anchor.BN
): Promise<boolean> {
  const [wishLikePda] = PublicKey.findProgramAddressSync(
    [
      Buffer.from("wish_like_v1"),
      liker.toBuffer(),
      creator.toBuffer(),
      wishId.toArrayLike(Buffer, "le", 8)
    ],
    program.programId
  );

  try {
    await program.account.wishLike.fetch(wishLikePda);
    return true; // 已点赞
  } catch (error) {
    return false; // 未点赞
  }
}
```

---

## 4. 取消点赞 (cancel_like_wish)

### 功能描述
用户取消对许愿的点赞，减少许愿的点赞数，并关闭点赞账户。

### 参数
- `wishId`: anchor.BN - 许愿ID

### 账户
| 账户名 | 类型 | 说明 |
|--------|------|------|
| wishLike | PDA (可写) | 点赞账户 |
| wish | PDA (可写) | 许愿账户 |
| creator | AccountInfo | 许愿创建者 |
| liker | Signer (可写) | 点赞者 |
| systemProgram | Program | 系统程序 |

### TypeScript 示例

```typescript
async function cancelLikeWish(
  program: anchor.Program,
  liker: anchor.web3.Keypair,
  creator: PublicKey,
  wishId: anchor.BN
): Promise<string> {
  // 计算 PDA
  const [wishLikePda] = PublicKey.findProgramAddressSync(
    [
      Buffer.from("wish_like_v1"),
      liker.publicKey.toBuffer(),
      creator.toBuffer(),
      wishId.toArrayLike(Buffer, "le", 8)
    ],
    program.programId
  );

  const [wishPda] = PublicKey.findProgramAddressSync(
    [
      Buffer.from("wish_v1"),
      creator.toBuffer(),
      wishId.toArrayLike(Buffer, "le", 8)
    ],
    program.programId
  );

  // 调用指令
  const tx = await program.methods
    .cancelLikeWish(wishId)
    .accounts({
      wishLike: wishLikePda,
      wish: wishPda,
      creator: creator,
      liker: liker.publicKey,
      systemProgram: anchor.web3.SystemProgram.programId,
    })
    .signers([liker])
    .rpc();

  console.log("✅ Like cancelled:", tx);
  return tx;
}

// 使用示例
const tx = await cancelLikeWish(
  program,
  likerKeypair,
  creatorPublicKey,
  wishId
);
```

---

## 完整互动流程示例

### 场景：用户完整体验流程

```typescript
async function completeUserJourney(
  program: anchor.Program,
  user: anchor.web3.Keypair
) {
  console.log("=== 用户互动完整流程 ===\n");

  // 1. 抽签
  console.log("1. 抽签...");
  const fortuneResult = await drawFortune(program, user);
  console.log(`   运势: ${getFortuneText(fortuneResult.fortune)}`);
  console.log(`   是否免费: ${fortuneResult.isFreeDraw}\n`);

  // 2. 许愿
  console.log("2. 许愿...");
  const wishResult = await createWish(
    program,
    user,
    "愿项目顺利上线",
    false
  );
  console.log(`   许愿ID: ${wishResult.wishId.toString()}`);
  console.log(`   是否获得御守: ${wishResult.isAmuletDropped}\n`);

  // 3. 查看许愿
  console.log("3. 查看许愿...");
  const wish = await getWish(program, user.publicKey, wishResult.wishId);
  console.log(`   点赞数: ${wish.likeCount}\n`);

  // 4. 其他用户点赞
  console.log("4. 其他用户点赞...");
  const otherUser = anchor.web3.Keypair.generate();
  await likeWish(program, otherUser, user.publicKey, wishResult.wishId);
  console.log(`   点赞成功\n`);

  // 5. 再次查看许愿
  console.log("5. 再次查看许愿...");
  const updatedWish = await getWish(program, user.publicKey, wishResult.wishId);
  console.log(`   点赞数: ${updatedWish.likeCount}\n`);

  console.log("=== 流程完成 ===");
}
```

---

## 注意事项

1. **每日限制**: 
   - 抽签：首次免费，后续消耗功德值
   - 许愿：前 3 次免费，后续消耗功德值

2. **功德值管理**: 
   - 确保用户有足够的功德值进行操作
   - 可通过烧香、捐赠等方式获得功德值

3. **许愿ID**: 
   - 建议使用时间戳作为许愿ID
   - 确保同一用户的许愿ID唯一

4. **内容哈希**: 
   - 使用 SHA-256 哈希算法
   - 32 字节数组格式

5. **点赞限制**: 
   - 每个用户对同一许愿只能点赞一次
   - 取消点赞会关闭点赞账户并退还租金

