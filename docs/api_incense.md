# 香品系统 API

## 1. 购买香 (buy_incense)

### 功能描述
用户使用 SOL 购买一种或多种香型。支持批量购买，最多同时购买 6 种不同香型。

### 参数

```typescript
interface BuyIncenseItem {
  incenseTypeId: number;  // 香型ID (1-6)
  quantity: number;       // 购买数量
  unitPrice: anchor.BN;   // 单价 (用于验证)
  subtotal: anchor.BN;    // 小计 (用于验证)
}

// 参数是数组
buyIncenseParams: BuyIncenseItem[]
```

### 账户
| 账户名 | 类型 | 说明 |
|--------|------|------|
| userIncenseState | PDA (可写, 自动初始化) | 用户香炉状态 |
| userState | PDA (可写, 自动初始化) | 用户状态 |
| templeTreasury | AccountInfo (可写) | 寺庙国库 |
| templeConfig | PDA (可写) | 寺庙配置 |
| user | Signer (可写) | 用户账户 |
| systemProgram | Program | 系统程序 |
| **剩余账户** | AccountMeta[] | 每个购买的香型对应一个 incenseTypeConfig |

### TypeScript 示例

```typescript
async function buyIncense(
  program: anchor.Program,
  user: anchor.web3.Keypair,
  buyItems: BuyIncenseItem[]
): Promise<string> {
  // 计算 PDA
  const [userStatePda] = PublicKey.findProgramAddressSync(
    [Buffer.from("user_state_v2"), user.publicKey.toBuffer()],
    program.programId
  );

  const [userIncenseStatePda] = PublicKey.findProgramAddressSync(
    [Buffer.from("user_incense_v1"), user.publicKey.toBuffer()],
    program.programId
  );

  const [templeConfigPda] = PublicKey.findProgramAddressSync(
    [Buffer.from("temple_config_v1")],
    program.programId
  );

  // 获取寺庙配置以获得 treasury 地址
  const templeConfig = await program.account.templeConfig.fetch(templeConfigPda);

  // 构建剩余账户列表
  const remainingAccounts = buyItems.map(item => {
    const [incenseTypeConfigPda] = PublicKey.findProgramAddressSync(
      [Buffer.from("incense_type_v1"), Buffer.from([item.incenseTypeId])],
      program.programId
    );
    return {
      pubkey: incenseTypeConfigPda,
      isSigner: false,
      isWritable: true,
    };
  });

  // 调用指令
  const tx = await program.methods
    .buyIncense(buyItems)
    .accounts({
      userIncenseState: userIncenseStatePda,
      userState: userStatePda,
      templeTreasury: templeConfig.treasury,
      templeConfig: templeConfigPda,
      user: user.publicKey,
      systemProgram: anchor.web3.SystemProgram.programId,
    })
    .remainingAccounts(remainingAccounts)
    .signers([user])
    .rpc();

  console.log("✅ Incense purchased:", tx);
  return tx;
}

// 使用示例：购买多种香
const buyItems = [
  {
    incenseTypeId: 1,
    quantity: 5,
    unitPrice: new anchor.BN(10_000_000),
    subtotal: new anchor.BN(50_000_000), // 5 * 0.01 = 0.05 SOL
  },
  {
    incenseTypeId: 2,
    quantity: 3,
    unitPrice: new anchor.BN(50_000_000),
    subtotal: new anchor.BN(150_000_000), // 3 * 0.05 = 0.15 SOL
  },
];

const tx = await buyIncense(program, userKeypair, buyItems);
```

### 命令行示例

```bash
# 使用 Anchor CLI
anchor run incense-buy

# 或使用测试脚本
ANCHOR_PROVIDER_URL=https://api.devnet.solana.com \
ANCHOR_WALLET=~/.config/solana/id.json \
npx mocha --require tsx tests/incense-buy.test.ts --timeout 60000
```

### 返回事件

```typescript
interface BuyIncenseEvent {
  user: PublicKey;              // 用户地址
  buyItems: BuyIncenseItem[];   // 购买项目列表
  totalSolAmount: anchor.BN;    // 总支付金额
  timestamp: number;            // 时间戳
  slot: number;                 // 区块槽位
}
```

### 查询用户香炉状态

```typescript
const [userIncenseStatePda] = PublicKey.findProgramAddressSync(
  [Buffer.from("user_incense_v1"), userPublicKey.toBuffer()],
  program.programId
);

const userIncenseState = await program.account.userIncenseState.fetch(
  userIncenseStatePda
);

console.log("User:", userIncenseState.user.toString());
console.log("Incense Balances:");
console.log("  清香 (ID 1):", userIncenseState.incenseBalance1.toString());
console.log("  檀香 (ID 2):", userIncenseState.incenseBalance2.toString());
console.log("  龙涎香 (ID 3):", userIncenseState.incenseBalance3.toString());
console.log("  太上灵香 (ID 4):", userIncenseState.incenseBalance4.toString());
console.log("  秘制香 (ID 5):", userIncenseState.incenseBalance5.toString());
```

### 限制说明

- 每次最多购买 **6 种**不同香型
- 每种香型的购买数量不能超过 `maxBuyPerTransaction`
- 用户必须有足够的 SOL 余额
- 只能购买 `purchasableWithSol = true` 的香型

---

## 2. 烧香 - 简化版 (burn_incense_simplied)

### 功能描述
用户直接使用 SOL 购买并烧香，无需先购买香。适合一次性烧香场景，自动铸造香品 NFT。

### 参数
- `incenseTypeId`: number - 香型ID (1-6)
- `amount`: number - 烧香数量 (1-10)
- `paymentAmount`: anchor.BN - 支付金额 (lamports)

### 返回值

```typescript
interface BurnIncenseResult {
  rewardIncenseValue: anchor.BN;  // 奖励的香火值
  rewardKarmaPoints: anchor.BN;   // 奖励的功德值
  incenseTypeId: number;          // 香型ID
  amount: number;                 // 烧香数量
  paymentAmount: anchor.BN;       // 支付金额
  currentTimestamp: number;       // 当前时间戳
}
```

### 账户
| 账户名 | 类型 | 说明 |
|--------|------|------|
| incenseTypeConfig | PDA (可写) | 香型配置 |
| templeAuthority | AccountInfo (可写) | 寺庙管理员 |
| templeConfig | PDA (可写) | 寺庙配置 |
| userState | PDA (可写, 自动初始化) | 用户状态 |
| user | Signer (可写) | 用户账户 |
| nftMintAccount | PDA (可写, 自动初始化) | NFT Mint 账户 |
| userNftAssociatedTokenAccount | Account (可写, 自动初始化) | 用户 NFT 关联账户 |
| metaAccount | UncheckedAccount (可写) | 元数据账户 |
| 其他程序账户 | - | Token, Metadata, AssociatedToken 程序 |

### TypeScript 示例

```typescript
import { getAssociatedTokenAddress } from "@solana/spl-token";

async function burnIncenseSimplified(
  program: anchor.Program,
  user: anchor.web3.Keypair,
  incenseTypeId: number,
  amount: number
): Promise<BurnIncenseResult> {
  // 获取香型配置以计算支付金额
  const [incenseTypeConfigPda] = PublicKey.findProgramAddressSync(
    [Buffer.from("incense_type_v1"), Buffer.from([incenseTypeId])],
    program.programId
  );

  const incenseConfig = await program.account.incenseTypeConfig.fetch(
    incenseTypeConfigPda
  );

  const paymentAmount = incenseConfig.pricePerUnit.muln(amount);

  // 计算其他 PDA
  const [templeConfigPda] = PublicKey.findProgramAddressSync(
    [Buffer.from("temple_config_v1")],
    program.programId
  );

  const templeConfig = await program.account.templeConfig.fetch(templeConfigPda);

  const [userStatePda] = PublicKey.findProgramAddressSync(
    [Buffer.from("user_state_v2"), user.publicKey.toBuffer()],
    program.programId
  );

  const [nftMintPda] = PublicKey.findProgramAddressSync(
    [
      Buffer.from("incense_nft_v1"),
      templeConfigPda.toBuffer(),
      Buffer.from([incenseTypeId])
    ],
    program.programId
  );

  const userNftAta = await getAssociatedTokenAddress(
    nftMintPda,
    user.publicKey
  );

  const TOKEN_METADATA_PROGRAM_ID = new PublicKey(
    "metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s"
  );

  const [metadataAccount] = PublicKey.findProgramAddressSync(
    [
      Buffer.from("metadata"),
      TOKEN_METADATA_PROGRAM_ID.toBuffer(),
      nftMintPda.toBuffer(),
    ],
    TOKEN_METADATA_PROGRAM_ID
  );

  // 调用指令
  const result = await program.methods
    .burnIncenseSimplied(incenseTypeId, amount, paymentAmount)
    .accounts({
      incenseTypeConfig: incenseTypeConfigPda,
      templeAuthority: templeConfig.authority,
      templeConfig: templeConfigPda,
      userState: userStatePda,
      user: user.publicKey,
      nftMintAccount: nftMintPda,
      userNftAssociatedTokenAccount: userNftAta,
      metaAccount: metadataAccount,
      rent: anchor.web3.SYSVAR_RENT_PUBKEY,
      systemProgram: anchor.web3.SystemProgram.programId,
      tokenProgram: anchor.utils.token.TOKEN_PROGRAM_ID,
      tokenMetadataProgram: TOKEN_METADATA_PROGRAM_ID,
      associatedTokenProgram: anchor.utils.token.ASSOCIATED_PROGRAM_ID,
    })
    .signers([user])
    .rpc();

  console.log("✅ Incense burned (simplified):", result);
  
  // 获取返回值需要解析交易日志或使用 view 方法
  return {
    rewardIncenseValue: incenseConfig.incenseValue * amount,
    rewardKarmaPoints: incenseConfig.karmaReward * amount,
    incenseTypeId,
    amount,
    paymentAmount,
    currentTimestamp: Date.now() / 1000,
  };
}

// 使用示例：烧 3 根清香
const result = await burnIncenseSimplified(program, userKeypair, 1, 3);
console.log("Karma Points Earned:", result.rewardKarmaPoints.toString());
console.log("Incense Value Earned:", result.rewardIncenseValue.toString());
```

### 命令行示例

```bash
# 使用 Anchor CLI
anchor run incense-burn-simplied

# 或使用测试脚本
ANCHOR_PROVIDER_URL=https://api.devnet.solana.com \
ANCHOR_WALLET=~/.config/solana/id.json \
npx mocha --require tsx tests/incense-burn-simplied.test.ts --timeout 60000
```

### 奖励计算

```typescript
// 功德值奖励 = 香型功德奖励 * 烧香数量
const karmaReward = incenseConfig.karmaReward * amount;

// 香火值奖励 = 香型香火值 * 烧香数量
const incenseValueReward = incenseConfig.incenseValue * amount;
```

### 限制说明

- 烧香数量范围：**1-10**
- 每日烧香次数限制：**3 次**（UTC 0:00 重置）
- 支付金额必须精确匹配：`pricePerUnit * amount`
- 用户必须有足够的 SOL 余额
- 香型必须处于激活状态

---

## 3. 烧香 - 标准版 (burn_incense)

### 功能描述
用户消耗已购买的香进行烧香，获得功德值和香火值，并铸造香品 NFT。需要先通过 `buy_incense` 购买香。

### 参数
- `incenseTypeId`: number - 香型ID (1-6)
- `amount`: number - 烧香数量 (1-10)

### 账户
与简化版类似，但需要额外的 `userIncenseState` 账户。

### TypeScript 示例

```typescript
async function burnIncense(
  program: anchor.Program,
  user: anchor.web3.Keypair,
  incenseTypeId: number,
  amount: number
): Promise<string> {
  // 计算 PDA（与简化版类似）
  const [incenseTypeConfigPda] = PublicKey.findProgramAddressSync(
    [Buffer.from("incense_type_v1"), Buffer.from([incenseTypeId])],
    program.programId
  );

  const [templeConfigPda] = PublicKey.findProgramAddressSync(
    [Buffer.from("temple_config_v1")],
    program.programId
  );

  const templeConfig = await program.account.templeConfig.fetch(templeConfigPda);

  const [userStatePda] = PublicKey.findProgramAddressSync(
    [Buffer.from("user_state_v2"), user.publicKey.toBuffer()],
    program.programId
  );

  const [userIncenseStatePda] = PublicKey.findProgramAddressSync(
    [Buffer.from("user_incense_v1"), user.publicKey.toBuffer()],
    program.programId
  );

  const [nftMintPda] = PublicKey.findProgramAddressSync(
    [
      Buffer.from("incense_nft_v1"),
      templeConfigPda.toBuffer(),
      Buffer.from([incenseTypeId])
    ],
    program.programId
  );

  const userNftAta = await getAssociatedTokenAddress(
    nftMintPda,
    user.publicKey
  );

  const TOKEN_METADATA_PROGRAM_ID = new PublicKey(
    "metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s"
  );

  const [metadataAccount] = PublicKey.findProgramAddressSync(
    [
      Buffer.from("metadata"),
      TOKEN_METADATA_PROGRAM_ID.toBuffer(),
      nftMintPda.toBuffer(),
    ],
    TOKEN_METADATA_PROGRAM_ID
  );

  // 调用指令
  const tx = await program.methods
    .burnIncense(incenseTypeId, amount)
    .accounts({
      incenseTypeConfig: incenseTypeConfigPda,
      templeAuthority: templeConfig.authority,
      templeConfig: templeConfigPda,
      userIncenseState: userIncenseStatePda,
      userState: userStatePda,
      user: user.publicKey,
      nftMintAccount: nftMintPda,
      userNftAssociatedTokenAccount: userNftAta,
      metaAccount: metadataAccount,
      rent: anchor.web3.SYSVAR_RENT_PUBKEY,
      systemProgram: anchor.web3.SystemProgram.programId,
      tokenProgram: anchor.utils.token.TOKEN_PROGRAM_ID,
      tokenMetadataProgram: TOKEN_METADATA_PROGRAM_ID,
      associatedTokenProgram: anchor.utils.token.ASSOCIATED_PROGRAM_ID,
    })
    .signers([user])
    .rpc();

  console.log("✅ Incense burned:", tx);
  return tx;
}

// 使用示例
const tx = await burnIncense(program, userKeypair, 1, 5);
```

### 命令行示例

```bash
# 使用 Anchor CLI
anchor run incense-burn

# 或使用测试脚本
ANCHOR_PROVIDER_URL=https://api.devnet.solana.com \
ANCHOR_WALLET=~/.config/solana/id.json \
npx mocha --require tsx tests/incense-burn.test.ts --timeout 60000
```

### 标准版 vs 简化版对比

| 特性 | 标准版 (burn_incense) | 简化版 (burn_incense_simplied) |
|------|----------------------|--------------------------------|
| 是否需要先购买香 | ✅ 需要 | ❌ 不需要 |
| 支付方式 | 消耗已购买的香 | 直接支付 SOL |
| 适用场景 | 批量购买后多次烧香 | 一次性烧香 |
| 账户数量 | 更多（需要 userIncenseState） | 较少 |
| Gas 费用 | 较低（已支付购买费用） | 较高（包含转账） |

---

## 4. 香品 NFT 说明

### NFT 特性

- **自动铸造**: 烧香时自动铸造对应数量的 NFT
- **不可转让**: NFT 作为烧香凭证，不可转让
- **元数据**: 包含香型信息、序列号等
- **收藏价值**: 不同稀有度的香品 NFT 具有不同的收藏价值

### 查询用户 NFT

```typescript
import { getAccount } from "@solana/spl-token";

async function getUserIncenseNFTs(
  connection: Connection,
  userPublicKey: PublicKey,
  incenseTypeId: number
): Promise<number> {
  const [templeConfigPda] = PublicKey.findProgramAddressSync(
    [Buffer.from("temple_config_v1")],
    programId
  );

  const [nftMintPda] = PublicKey.findProgramAddressSync(
    [
      Buffer.from("incense_nft_v1"),
      templeConfigPda.toBuffer(),
      Buffer.from([incenseTypeId])
    ],
    programId
  );

  const userNftAta = await getAssociatedTokenAddress(
    nftMintPda,
    userPublicKey
  );

  try {
    const tokenAccount = await getAccount(connection, userNftAta);
    return Number(tokenAccount.amount);
  } catch (error) {
    return 0; // 账户不存在，返回 0
  }
}

// 使用示例
const nftCount = await getUserIncenseNFTs(connection, userPublicKey, 1);
console.log("清香 NFT 数量:", nftCount);
```

---

## 完整流程示例

### 场景 1：购买后烧香

```typescript
// 1. 购买香
const buyItems = [{
  incenseTypeId: 1,
  quantity: 10,
  unitPrice: new anchor.BN(10_000_000),
  subtotal: new anchor.BN(100_000_000),
}];
await buyIncense(program, user, buyItems);

// 2. 烧香（可多次）
await burnIncense(program, user, 1, 3);
await burnIncense(program, user, 1, 3);
await burnIncense(program, user, 1, 3); // 达到每日限制

// 3. 查询余额
const userIncenseState = await program.account.userIncenseState.fetch(userIncenseStatePda);
console.log("剩余香数:", userIncenseState.incenseBalance1.toString()); // 1 根
```

### 场景 2：直接烧香

```typescript
// 一次性烧香，无需先购买
const result = await burnIncenseSimplified(program, user, 2, 5);
console.log("获得功德值:", result.rewardKarmaPoints.toString());
console.log("获得香火值:", result.rewardIncenseValue.toString());
```

---

## 注意事项

1. **每日限制**: 每日烧香次数限制为 3 次，UTC 0:00 重置
2. **余额检查**: 标准版烧香前需确保有足够的香余额
3. **支付验证**: 购买和简化版烧香时，支付金额必须精确匹配
4. **NFT 存储**: NFT 会自动存储在用户的关联 Token 账户中
5. **Gas 优化**: 批量购买后多次烧香比每次都用简化版更省 Gas

