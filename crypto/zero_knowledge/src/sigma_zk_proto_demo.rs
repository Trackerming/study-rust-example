/// Sigma 零知识证明:知道秘密s ，且与公开输入 Q 满足离散对数关系 Q =w*G
/// 1. 承诺：P选择随机数r，计算R = r*G，发送R
/// 2. 挑战：V发送随机数e
/// 3. 响应：P计算响应 z = r+e*w
/// 4. 验证：V验证 z*G 是否等于 R+e*Q
pub struct SigmaZK {}
