# 寺庙和用户管理 API

## 1. 寺庙初始化 (init_temple)

### 功能描述
初始化寺庙全局配置，只能由管理员执行一次。

### 参数
- `treasury`: Pubkey - 寺庙国库地址

### 账户
| 账户名 | 类型 | 说明 |
|--------|------|------|
| templeConfig | PDA (可写, 初始化) | 寺庙配置账户 |
| authority | Signer (可写) | 管理员账户 |
| systemProgram | Program | 系统程序 |

### TypeScript 示例

```typescript
import * as anchor from "@coral-xyz/anchor";
import { PublicKey } from "@solana/web3.js";

async function initTemple(
  program: anchor.Program,
  authority: anchor.web3.Keypair,
  treasury: PublicKey
): Promise<string> {
  // 计算寺庙配置 PDA
  const [templeConfigPda] = PublicKey.findProgramAddressSync(
    [Buffer.from("temple_config_v1")],
    program.programId
  );

  // 调用指令
  const tx = await program.methods
    .initTemple(treasury)
    .accounts({
      templeConfig: templeConfigPda,
      authority: authority.publicKey,
      systemProgram: anchor.web3.SystemProgram.programId,
    })
    .signers([authority])
    .rpc();

  console.log("✅ Temple initialized:", tx);
  return tx;
}

// 使用示例
const treasury = new PublicKey("YOUR_TREASURY_ADDRESS");
const tx = await initTemple(program, authorityKeypair, treasury);
```

### 命令行示例

```bash
# 使用 Anchor CLI
cd anchor
anchor run temple-init

# 或使用测试脚本
ANCHOR_PROVIDER_URL=https://api.devnet.solana.com \
ANCHOR_WALLET=~/.config/solana/id.json \
npx mocha --require tsx tests/temple-init.test.ts --timeout 60000
```

### 返回事件

```typescript
interface TempleInitEvent {
  templeConfig: PublicKey;  // 寺庙配置地址
  authority: PublicKey;     // 管理员地址
  templeLevel: number;      // 寺庙等级
  timestamp: number;        // 时间戳
}
```

### 查询寺庙状态

```bash
# 获取寺庙配置 PDA 地址
solana address --program-id 81BWs7RGtN2EEvaGWZe8EQ8nhswHTHVzYUn5iPFoRr9o \
  --seed temple_config_v1

# 查询账户数据
solana account <TEMPLE_CONFIG_PDA> --url https://api.devnet.solana.com
```

```typescript
// TypeScript 查询
const [templeConfigPda] = PublicKey.findProgramAddressSync(
  [Buffer.from("temple_config_v1")],
  program.programId
);

const templeConfig = await program.account.templeConfig.fetch(templeConfigPda);
console.log("Authority:", templeConfig.authority.toString());
console.log("Temple Level:", templeConfig.templeLevel);
console.log("Total Incense Value:", templeConfig.totalIncenseValue.toString());
console.log("Total Draws:", templeConfig.totalDraws.toString());
console.log("Total Wishes:", templeConfig.totalWishes.toString());
```

---

## 2. 用户初始化 (init_user)

### 功能描述
初始化用户状态账户，记录用户的功德值、香火值等信息。每个用户只需初始化一次。

### 参数
无

### 账户
| 账户名 | 类型 | 说明 |
|--------|------|------|
| userState | PDA (可写, 初始化) | 用户状态账户 |
| user | Signer (可写) | 用户账户 |
| systemProgram | Program | 系统程序 |

### TypeScript 示例

```typescript
async function initUser(
  program: anchor.Program,
  user: anchor.web3.Keypair
): Promise<string> {
  // 计算用户状态 PDA
  const [userStatePda] = PublicKey.findProgramAddressSync(
    [Buffer.from("user_state_v1"), user.publicKey.toBuffer()],
    program.programId
  );

  // 调用指令
  const tx = await program.methods
    .initUser()
    .accounts({
      userState: userStatePda,
      user: user.publicKey,
      systemProgram: anchor.web3.SystemProgram.programId,
    })
    .signers([user])
    .rpc();

  console.log("✅ User initialized:", tx);
  return tx;
}

// 使用示例
const tx = await initUser(program, userKeypair);
```

### 命令行示例

```bash
# 使用 Anchor CLI
anchor run user-init

# 或使用测试脚本
ANCHOR_PROVIDER_URL=https://api.devnet.solana.com \
ANCHOR_WALLET=~/.config/solana/id.json \
npx mocha --require tsx tests/user-init.test.ts --timeout 60000
```

### 返回事件

```typescript
interface UserInitEvent {
  userState: PublicKey;  // 用户状态地址
  user: PublicKey;       // 用户地址
  timestamp: number;     // 时间戳
}
```

### 查询用户状态

```typescript
// 计算用户状态 PDA
const [userStatePda] = PublicKey.findProgramAddressSync(
  [Buffer.from("user_state_v1"), userPublicKey.toBuffer()],
  program.programId
);

// 查询用户状态
const userState = await program.account.userState.fetch(userStatePda);

console.log("User:", userState.user.toString());
console.log("Karma Points:", userState.karmaPoints.toString());
console.log("Incense Value:", userState.incenseValue.toString());
console.log("Total Spending:", userState.totalSpending.toString());
console.log("Total Burn Count:", userState.totalBurnCount.toString());
console.log("Total Draw Count:", userState.totalDrawCount.toString());
console.log("Total Wish Count:", userState.totalWishCount.toString());
console.log("Daily Burn Count:", userState.dailyBurnCount);
console.log("Daily Draw Count:", userState.dailyDrawCount);
console.log("Daily Wish Count:", userState.dailyWishCount);
```

### 每日限制说明

用户状态包含每日限制机制：

- **每日烧香次数**: 默认 3 次
- **每日抽签次数**: 无限制（首次免费，后续消耗功德值）
- **每日许愿次数**: 前 3 次免费，后续消耗功德值

系统会在每日 UTC 0:00 自动重置计数器。

### 完整示例：检查并初始化用户

```typescript
async function ensureUserInitialized(
  program: anchor.Program,
  user: anchor.web3.Keypair
): Promise<PublicKey> {
  const [userStatePda] = PublicKey.findProgramAddressSync(
    [Buffer.from("user_state_v1"), user.publicKey.toBuffer()],
    program.programId
  );

  try {
    // 尝试获取用户状态
    const userState = await program.account.userState.fetch(userStatePda);
    console.log("✅ User already initialized");
    return userStatePda;
  } catch (error) {
    // 用户未初始化，执行初始化
    console.log("🚀 Initializing user...");
    await initUser(program, user);
    console.log("✅ User initialized successfully");
    return userStatePda;
  }
}
```

---

## 3. 香型初始化 (init_incense_type)

### 功能描述
创建新的香型配置，定义香的属性、价格、奖励等。只能由管理员执行。

### 参数

```typescript
interface InitializeIncenseTypeParams {
  incenseTypeId: number;        // 香型ID (1-6)
  name: string;                 // 香名称 (最大32字符)
  description: string;          // 描述 (最大128字符)
  pricePerUnit: anchor.BN;     // 单价 (lamports)
  karmaReward: number;         // 功德奖励
  incenseValue: number;        // 香火值
  purchasableWithSol: boolean; // 是否可用SOL购买
  maxBuyPerTransaction: number;// 单次最大购买数量
  isActive: boolean;           // 是否激活
  rarity: IncenseRarity;       // 稀有度
  nftCollection: PublicKey;    // NFT集合地址
  metadataUriTemplate: string; // 元数据URI模板 (最大200字符)
}

// 稀有度枚举
enum IncenseRarity {
  Common = 1,    // 普通
  Rare = 2,      // 稀有
  Epic = 3,      // 史诗
  Legendary = 4, // 传说
}
```

### 账户
| 账户名 | 类型 | 说明 |
|--------|------|------|
| incenseTypeConfig | PDA (可写, 初始化) | 香型配置账户 |
| templeConfig | PDA (可写) | 寺庙配置账户 |
| authority | Signer (可写) | 管理员账户 |
| systemProgram | Program | 系统程序 |

### TypeScript 示例

```typescript
async function initIncenseType(
  program: anchor.Program,
  authority: anchor.web3.Keypair,
  params: InitializeIncenseTypeParams
): Promise<string> {
  // 计算香型配置 PDA
  const [incenseTypeConfigPda] = PublicKey.findProgramAddressSync(
    [Buffer.from("incense_type_v1"), Buffer.from([params.incenseTypeId])],
    program.programId
  );

  // 计算寺庙配置 PDA
  const [templeConfigPda] = PublicKey.findProgramAddressSync(
    [Buffer.from("temple_config_v1")],
    program.programId
  );

  // 调用指令
  const tx = await program.methods
    .initIncenseType(params)
    .accounts({
      incenseTypeConfig: incenseTypeConfigPda,
      templeConfig: templeConfigPda,
      authority: authority.publicKey,
      systemProgram: anchor.web3.SystemProgram.programId,
    })
    .signers([authority])
    .rpc();

  console.log("✅ Incense type initialized:", tx);
  return tx;
}

// 使用示例：初始化清香
const params = {
  incenseTypeId: 1,
  name: "清香",
  description: "清淡香味，适合日常冥想，带来内心平静",
  pricePerUnit: new anchor.BN(10_000_000), // 0.01 SOL
  karmaReward: 10,
  incenseValue: 100,
  purchasableWithSol: true,
  maxBuyPerTransaction: 10,
  isActive: true,
  rarity: { common: {} },
  nftCollection: PublicKey.default,
  metadataUriTemplate: "https://api.solji.com/metadata/qingxiang/{sequence}",
};

const tx = await initIncenseType(program, authorityKeypair, params);
```

### 预定义香型

```typescript
// 1. 清香 - 普通
const qingxiang = {
  incenseTypeId: 1,
  name: "清香",
  pricePerUnit: new anchor.BN(10_000_000), // 0.01 SOL
  karmaReward: 10,
  incenseValue: 100,
  rarity: { common: {} },
};

// 2. 檀香 - 稀有
const tanxiang = {
  incenseTypeId: 2,
  name: "檀香",
  pricePerUnit: new anchor.BN(50_000_000), // 0.05 SOL
  karmaReward: 60,
  incenseValue: 500,
  rarity: { rare: {} },
};

// 3. 龙涎香 - 史诗
const longxianxiang = {
  incenseTypeId: 3,
  name: "龙涎香",
  pricePerUnit: new anchor.BN(100_000_000), // 0.1 SOL
  karmaReward: 1200,
  incenseValue: 3100,
  rarity: { epic: {} },
};

// 4. 太上灵香 - 史诗
const taishangxiang = {
  incenseTypeId: 4,
  name: "太上灵香",
  pricePerUnit: new anchor.BN(300_000_000), // 0.3 SOL
  karmaReward: 3400,
  incenseValue: 9000,
  rarity: { epic: {} },
};

// 5. 秘制香 - 传说（捐助解锁）
const mizhixiang = {
  incenseTypeId: 5,
  name: "秘制香",
  pricePerUnit: new anchor.BN(5_000_000_000), // 5 SOL
  karmaReward: 100000,
  incenseValue: 300000,
  purchasableWithSol: false, // 不可直接购买
  rarity: { legendary: {} },
};
```

### 命令行示例

```bash
# 使用 Anchor CLI
anchor run incense-init

# 或使用测试脚本
ANCHOR_PROVIDER_URL=https://api.devnet.solana.com \
ANCHOR_WALLET=~/.config/solana/id.json \
npx mocha --require tsx tests/incense-init.test.ts --timeout 60000
```

### 返回事件

```typescript
interface IncenseInitEvent {
  incenseTypeConfig: PublicKey;  // 香型配置地址
  incenseTypeId: number;         // 香型ID
  name: string;                  // 名称
  pricePerUnit: anchor.BN;       // 单价
  karmaReward: number;           // 功德奖励
  incenseValue: number;          // 香火值
  isActive: boolean;             // 是否激活
  timestamp: number;             // 时间戳
}
```

### 查询香型配置

```typescript
// 查询指定香型
const [incenseTypeConfigPda] = PublicKey.findProgramAddressSync(
  [Buffer.from("incense_type_v1"), Buffer.from([1])], // 香型ID: 1
  program.programId
);

const incenseConfig = await program.account.incenseTypeConfig.fetch(
  incenseTypeConfigPda
);

console.log("Incense Type ID:", incenseConfig.incenseTypeId);
console.log("Name:", incenseConfig.name);
console.log("Description:", incenseConfig.description);
console.log("Price:", incenseConfig.pricePerUnit.toString(), "lamports");
console.log("Karma Reward:", incenseConfig.karmaReward);
console.log("Incense Value:", incenseConfig.incenseValue);
console.log("Is Active:", incenseConfig.isActive);
console.log("Total Minted:", incenseConfig.totalMinted.toString());
```

---

## 注意事项

1. **权限控制**: 寺庙初始化和香型初始化只能由管理员执行
2. **一次性操作**: 寺庙配置和用户状态只能初始化一次
3. **PDA 唯一性**: 每个 PDA 地址由种子唯一确定，确保数据不会重复
4. **费用**: 所有初始化操作需要支付账户租金，由签名者支付
5. **网络**: 建议在 Devnet 上测试，确认无误后再部署到 Mainnet

