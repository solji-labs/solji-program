# 捐赠和 NFT 系统 API

## 1. 捐赠 (donate_fund)

### 功能描述
用户向寺庙捐赠 SOL，获得功德值和香火值奖励。根据捐赠金额自动铸造或升级徽章 NFT。捐赠 ≥5 SOL 还会空投高级香型。

### 参数
- `amount`: anchor.BN - 捐赠金额 (lamports)

### 返回值

```typescript
interface DonateFundResult {
  rewardIncenseValue: anchor.BN;  // 奖励的香火值
  rewardKarmaPoints: anchor.BN;   // 奖励的功德值
  donationAmount: anchor.BN;      // 捐赠金额
  currentTimestamp: number;       // 当前时间戳
}
```

### 账户
| 账户名 | 类型 | 说明 |
|--------|------|------|
| user | Signer (可写) | 用户账户 |
| userState | PDA (可写, 自动初始化) | 用户状态 |
| userIncenseState | PDA (可写, 自动初始化) | 用户香炉状态 |
| userDonationState | PDA (可写, 自动初始化) | 用户捐赠状态 |
| templeTreasury | AccountInfo (可写) | 寺庙国库 |
| templeConfig | PDA (可写) | 寺庙配置 |
| nftMintAccount | PDA (可写, 自动初始化) | 徽章 NFT Mint |
| userNftAssociatedTokenAccount | Account (可写, 自动初始化) | 用户 NFT 关联账户 |
| metaAccount | UncheckedAccount (可写) | 元数据账户 |
| 其他程序账户 | - | Token, Metadata, AssociatedToken 程序 |

### TypeScript 示例

```typescript
import { getAssociatedTokenAddress } from "@solana/spl-token";

async function donateFund(
  program: anchor.Program,
  user: anchor.web3.Keypair,
  amountSol: number
): Promise<DonateFundResult> {
  const amount = new anchor.BN(amountSol * anchor.web3.LAMPORTS_PER_SOL);

  // 计算 PDA
  const [userStatePda] = PublicKey.findProgramAddressSync(
    [Buffer.from("user_state_v1"), user.publicKey.toBuffer()],
    program.programId
  );

  const [userIncenseStatePda] = PublicKey.findProgramAddressSync(
    [Buffer.from("user_incense_v1"), user.publicKey.toBuffer()],
    program.programId
  );

  const [userDonationStatePda] = PublicKey.findProgramAddressSync(
    [Buffer.from("user_donation_v1"), user.publicKey.toBuffer()],
    program.programId
  );

  const [templeConfigPda] = PublicKey.findProgramAddressSync(
    [Buffer.from("temple_config_v1")],
    program.programId
  );

  const templeConfig = await program.account.templeConfig.fetch(templeConfigPda);

  // 徽章 NFT Mint PDA
  const [badgeNftMintPda] = PublicKey.findProgramAddressSync(
    [
      Buffer.from("badge_nft_v1"),
      templeConfigPda.toBuffer(),
      user.publicKey.toBuffer()
    ],
    program.programId
  );

  const userNftAta = await getAssociatedTokenAddress(
    badgeNftMintPda,
    user.publicKey
  );

  const TOKEN_METADATA_PROGRAM_ID = new PublicKey(
    "metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s"
  );

  const [metadataAccount] = PublicKey.findProgramAddressSync(
    [
      Buffer.from("metadata"),
      TOKEN_METADATA_PROGRAM_ID.toBuffer(),
      badgeNftMintPda.toBuffer(),
    ],
    TOKEN_METADATA_PROGRAM_ID
  );

  // 调用指令
  const tx = await program.methods
    .donateFund(amount)
    .accounts({
      user: user.publicKey,
      userState: userStatePda,
      userIncenseState: userIncenseStatePda,
      userDonationState: userDonationStatePda,
      templeTreasury: templeConfig.treasury,
      templeConfig: templeConfigPda,
      nftMintAccount: badgeNftMintPda,
      userNftAssociatedTokenAccount: userNftAta,
      metaAccount: metadataAccount,
      tokenProgram: anchor.utils.token.TOKEN_PROGRAM_ID,
      systemProgram: anchor.web3.SystemProgram.programId,
      tokenMetadataProgram: TOKEN_METADATA_PROGRAM_ID,
      associatedTokenProgram: anchor.utils.token.ASSOCIATED_PROGRAM_ID,
      rent: anchor.web3.SYSVAR_RENT_PUBKEY,
    })
    .signers([user])
    .rpc();

  console.log("✅ Donation successful:", tx);

  // 计算奖励（实际应从交易日志解析）
  const { karmaReward, incenseValueReward } = calculateDonationRewards(amount);

  return {
    rewardIncenseValue: incenseValueReward,
    rewardKarmaPoints: karmaReward,
    donationAmount: amount,
    currentTimestamp: Date.now() / 1000,
  };
}

// 奖励计算函数
function calculateDonationRewards(amount: anchor.BN): {
  karmaReward: anchor.BN;
  incenseValueReward: anchor.BN;
} {
  // 功德值 = 捐赠金额 / 10,000,000 * 100
  const karmaReward = amount.divn(10_000_000).muln(100);
  
  // 香火值 = 捐赠金额 / 10,000,000 * 1000
  const incenseValueReward = amount.divn(10_000_000).muln(1000);

  return { karmaReward, incenseValueReward };
}

// 使用示例：捐赠 1 SOL
const result = await donateFund(program, userKeypair, 1);
console.log("获得功德值:", result.rewardKarmaPoints.toString());
console.log("获得香火值:", result.rewardIncenseValue.toString());
```

### 命令行示例

```bash
# 使用 Anchor CLI
anchor run donation

# 或使用测试脚本
ANCHOR_PROVIDER_URL=https://api.devnet.solana.com \
ANCHOR_WALLET=~/.config/solana/id.json \
npx mocha --require tsx tests/donation.test.ts --timeout 60000
```

### 捐赠等级和徽章

| 等级 | 累计捐赠 | 徽章名称 | 特殊奖励 |
|------|---------|---------|---------|
| 1 | 0.05 SOL | 青铜护法 | - |
| 2 | 0.2 SOL | 白银护法 | - |
| 3 | 1 SOL | 黄金护法 | - |
| 4 | 5 SOL | 钻石护法 | 空投高级香型 |

### 徽章 NFT 特性

- **自动铸造**: 首次捐赠自动铸造徽章 NFT
- **自动升级**: 达到更高等级时自动更新 NFT 元数据
- **唯一性**: 每个用户只有一个徽章 NFT
- **不可转让**: 徽章 NFT 作为荣誉凭证，不可转让
- **元数据**: 包含捐赠等级、累计金额等信息

### 高级香型空投

当捐赠金额 ≥5 SOL 时，自动空投高级香型：

```typescript
// 空投规则
if (donationAmount >= 5 SOL) {
  // 空投秘制香（香型ID: 5）
  // 数量根据捐赠金额计算
  const airdropAmount = Math.floor(donationAmount / 5);
  // 自动添加到用户香炉状态
}
```

### 查询捐赠状态

```typescript
async function getUserDonationState(
  program: anchor.Program,
  userPublicKey: PublicKey
) {
  const [userDonationStatePda] = PublicKey.findProgramAddressSync(
    [Buffer.from("user_donation_v1"), userPublicKey.toBuffer()],
    program.programId
  );

  const donationState = await program.account.userDonationState.fetch(
    userDonationStatePda
  );

  console.log("User:", donationState.user.toString());
  console.log("Total Donation:", donationState.totalDonationAmount.toString(), "lamports");
  console.log("Donation Count:", donationState.totalDonationCount.toString());
  console.log("Donation Level:", donationState.donationLevel);
  console.log("Last Donation:", new Date(donationState.lastDonationAt.toNumber() * 1000));
  console.log("Can Mint Buddha NFT:", donationState.canMintBuddhaNft);
  console.log("Has Minted Buddha NFT:", donationState.hasMintedBuddhaNft);
  console.log("Has Minted Badge NFT:", donationState.hasMintedBadgeNft);

  return donationState;
}
```

---

## 2. 铸造佛像 NFT (mint_buddha_nft)

### 功能描述
用户铸造限量版佛像 NFT。需要满足特定条件（如达到一定捐赠等级）。全局限量 10,000 个。

### 参数
无

### 账户
| 账户名 | 类型 | 说明 |
|--------|------|------|
| buddhaNftAccount | PDA (可写, 初始化) | 佛像 NFT 账户 |
| userState | PDA (可写) | 用户状态 |
| userDonationState | PDA (可写, 自动初始化) | 用户捐赠状态 |
| user | Signer (可写) | 用户账户 |
| templeConfig | PDA (可写) | 寺庙配置 |
| nftMintAccount | PDA (可写, 自动初始化) | NFT Mint 账户 |
| userNftAssociatedTokenAccount | Account (可写, 自动初始化) | 用户 NFT 关联账户 |
| metaAccount | UncheckedAccount (可写) | 元数据账户 |
| 其他程序账户 | - | Token, Metadata, AssociatedToken 程序 |

### TypeScript 示例

```typescript
async function mintBuddhaNft(
  program: anchor.Program,
  user: anchor.web3.Keypair
): Promise<string> {
  // 计算 PDA
  const [templeConfigPda] = PublicKey.findProgramAddressSync(
    [Buffer.from("temple_config_v1")],
    program.programId
  );

  const [buddhaNftAccountPda] = PublicKey.findProgramAddressSync(
    [
      Buffer.from("buddha_nft_v1"),
      Buffer.from("account"),
      templeConfigPda.toBuffer(),
      user.publicKey.toBuffer()
    ],
    program.programId
  );

  const [userStatePda] = PublicKey.findProgramAddressSync(
    [Buffer.from("user_state_v1"), user.publicKey.toBuffer()],
    program.programId
  );

  const [userDonationStatePda] = PublicKey.findProgramAddressSync(
    [Buffer.from("user_donation_v1"), user.publicKey.toBuffer()],
    program.programId
  );

  const [nftMintPda] = PublicKey.findProgramAddressSync(
    [
      Buffer.from("buddha_nft_v1"),
      templeConfigPda.toBuffer(),
      user.publicKey.toBuffer()
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
    .mintBuddhaNft()
    .accounts({
      buddhaNftAccount: buddhaNftAccountPda,
      userState: userStatePda,
      userDonationState: userDonationStatePda,
      user: user.publicKey,
      templeConfig: templeConfigPda,
      nftMintAccount: nftMintPda,
      userNftAssociatedTokenAccount: userNftAta,
      metaAccount: metadataAccount,
      systemProgram: anchor.web3.SystemProgram.programId,
      tokenProgram: anchor.utils.token.TOKEN_PROGRAM_ID,
      tokenMetadataProgram: TOKEN_METADATA_PROGRAM_ID,
      associatedTokenProgram: anchor.utils.token.ASSOCIATED_PROGRAM_ID,
      rent: anchor.web3.SYSVAR_RENT_PUBKEY,
    })
    .signers([user])
    .rpc();

  console.log("✅ Buddha NFT minted:", tx);
  return tx;
}

// 使用示例
const tx = await mintBuddhaNft(program, userKeypair);
```

### 命令行示例

```bash
ANCHOR_PROVIDER_URL=https://api.devnet.solana.com \
ANCHOR_WALLET=~/.config/solana/id.json \
npx mocha --require tsx tests/mint-buddha-nft.test.ts --timeout 60000
```

### 铸造条件

```typescript
// 检查用户是否可以铸造佛像 NFT
async function canMintBuddhaNft(
  program: anchor.Program,
  userPublicKey: PublicKey
): Promise<boolean> {
  try {
    const [userDonationStatePda] = PublicKey.findProgramAddressSync(
      [Buffer.from("user_donation_v1"), userPublicKey.toBuffer()],
      program.programId
    );

    const donationState = await program.account.userDonationState.fetch(
      userDonationStatePda
    );

    // 检查条件
    const canMint = donationState.canMintBuddhaNft;
    const hasMinted = donationState.hasMintedBuddhaNft;

    return canMint && !hasMinted;
  } catch (error) {
    return false;
  }
}
```

### 佛像 NFT 特性

- **限量发行**: 全局限量 10,000 个
- **序列号**: 每个 NFT 有唯一的序列号
- **不可转让**: 铸造后账户被冻结，不可转让
- **荣誉象征**: 代表用户对寺庙的贡献
- **元数据**: 包含序列号、铸造时间等信息

### 查询佛像 NFT

```typescript
async function getBuddhaNft(
  program: anchor.Program,
  userPublicKey: PublicKey
) {
  const [templeConfigPda] = PublicKey.findProgramAddressSync(
    [Buffer.from("temple_config_v1")],
    program.programId
  );

  const [buddhaNftAccountPda] = PublicKey.findProgramAddressSync(
    [
      Buffer.from("buddha_nft_v1"),
      Buffer.from("account"),
      templeConfigPda.toBuffer(),
      userPublicKey.toBuffer()
    ],
    program.programId
  );

  try {
    const buddhaNft = await program.account.buddhaNft.fetch(buddhaNftAccountPda);

    console.log("Owner:", buddhaNft.owner.toString());
    console.log("Mint:", buddhaNft.mint.toString());
    console.log("Serial Number:", buddhaNft.serialNumber);
    console.log("Minted At:", new Date(buddhaNft.mintedAt.toNumber() * 1000));

    return buddhaNft;
  } catch (error) {
    console.log("User has not minted Buddha NFT");
    return null;
  }
}
```

---

## 3. NFT 元数据说明

### 徽章 NFT 元数据

```json
{
  "name": "钻石护法",
  "symbol": "TEMPLE",
  "uri": "https://api.foxverse.co/temple/badge/4/metadata.json",
  "seller_fee_basis_points": 0,
  "properties": {
    "category": "image",
    "files": [
      {
        "uri": "https://api.foxverse.co/temple/badge/4/image.png",
        "type": "image/png"
      }
    ]
  },
  "attributes": [
    {
      "trait_type": "Level",
      "value": "4"
    },
    {
      "trait_type": "Total Donation",
      "value": "5.0 SOL"
    }
  ]
}
```

### 佛像 NFT 元数据

```json
{
  "name": "Buddha NFT #1234",
  "symbol": "BUDDHA",
  "uri": "https://api.foxverse.co/temple/buddha/metadata.json",
  "seller_fee_basis_points": 0,
  "properties": {
    "category": "image",
    "files": [
      {
        "uri": "https://api.foxverse.co/temple/buddha/image.png",
        "type": "image/png"
      }
    ]
  },
  "attributes": [
    {
      "trait_type": "Serial Number",
      "value": "1234"
    },
    {
      "trait_type": "Minted At",
      "value": "2025-10-22"
    }
  ]
}
```

### 香品 NFT 元数据

```json
{
  "name": "清香 #001",
  "symbol": "INCENSE",
  "uri": "https://api.solji.com/metadata/qingxiang/1",
  "seller_fee_basis_points": 0,
  "properties": {
    "category": "image",
    "files": [
      {
        "uri": "https://api.solji.com/images/qingxiang.png",
        "type": "image/png"
      }
    ]
  },
  "attributes": [
    {
      "trait_type": "Type",
      "value": "清香"
    },
    {
      "trait_type": "Rarity",
      "value": "Common"
    },
    {
      "trait_type": "Sequence",
      "value": "1"
    }
  ]
}
```

---

## 完整捐赠流程示例

```typescript
async function completeDonationFlow(
  program: anchor.Program,
  user: anchor.web3.Keypair
) {
  console.log("=== 捐赠完整流程 ===\n");

  // 1. 小额捐赠（0.05 SOL）- 获得青铜护法
  console.log("1. 捐赠 0.05 SOL...");
  let result = await donateFund(program, user, 0.05);
  console.log(`   获得功德值: ${result.rewardKarmaPoints.toString()}`);
  console.log(`   获得香火值: ${result.rewardIncenseValue.toString()}\n`);

  // 2. 查询捐赠状态
  console.log("2. 查询捐赠状态...");
  let donationState = await getUserDonationState(program, user.publicKey);
  console.log(`   当前等级: ${donationState.donationLevel}`);
  console.log(`   累计捐赠: ${donationState.totalDonationAmount.toString()} lamports\n`);

  // 3. 继续捐赠达到钻石护法（5 SOL）
  console.log("3. 捐赠 5 SOL...");
  result = await donateFund(program, user, 5);
  console.log(`   获得功德值: ${result.rewardKarmaPoints.toString()}`);
  console.log(`   获得香火值: ${result.rewardIncenseValue.toString()}`);
  console.log(`   空投高级香型\n`);

  // 4. 查询更新后的状态
  console.log("4. 查询更新后的状态...");
  donationState = await getUserDonationState(program, user.publicKey);
  console.log(`   当前等级: ${donationState.donationLevel}`);
  console.log(`   累计捐赠: ${donationState.totalDonationAmount.toString()} lamports`);
  console.log(`   可铸造佛像NFT: ${donationState.canMintBuddhaNft}\n`);

  // 5. 铸造佛像 NFT（如果符合条件）
  if (await canMintBuddhaNft(program, user.publicKey)) {
    console.log("5. 铸造佛像 NFT...");
    await mintBuddhaNft(program, user);
    console.log(`   佛像 NFT 铸造成功\n`);
  }

  console.log("=== 流程完成 ===");
}
```

---

## 注意事项

1. **捐赠金额**: 建议最小捐赠 0.05 SOL 以获得徽章
2. **NFT 限量**: 佛像 NFT 全局限量 10,000 个，先到先得
3. **账户租金**: 所有 NFT 相关操作需要支付账户租金
4. **元数据更新**: 徽章 NFT 会随着捐赠等级自动更新
5. **高级香型**: 捐赠 ≥5 SOL 自动空投秘制香，无需额外操作

