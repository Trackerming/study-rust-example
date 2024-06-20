template Mul2(){
  // 定义信号
  signal private input a;
  signal private input b;
  signal output c;
  // 表达式
  c <== a*b; 
} 
component main = Mul2();
