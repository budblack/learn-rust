#include <iostream>


extern "C"{
  void sayHello() {
    std::cout << "Hello" << std::endl;
  }
}