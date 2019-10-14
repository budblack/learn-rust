#include <iostream>

extern "C"{
  int db(int a);
  void sayHello();
}

int db(int a) {
  return a * 2;
}

// void sayHello() {
//   std::cout << "Hello" << std::endl;
// }
