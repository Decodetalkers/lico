#include <string>
#include <iostream>
using namespace std;
namespace Solution {
    bool isPalidrome(int x){
        if (x<0) {
            return false;
        }
        string temp = to_string(x);
        for(int i=0, j=temp.length()-1;i<=j; i++,j--){
            if (temp[i]!=temp[j]) {
                return false;
            }
        }
        return true;
    }
    namespace lico {
        bool isPalindrome(int x) {
            if (x < 0 || (x % 10 == 0 && x != 0)) {
                return false;
            }

            int revertedNumber = 0;
            while (x > revertedNumber) {
                revertedNumber = revertedNumber * 10 + x % 10;
                x /= 10;
            }
            return x == revertedNumber || x == revertedNumber / 10;
        }
    }
}
int main()
{
    std::cout << Solution::isPalidrome(12321) << std::endl;
    std::cout << Solution::lico::isPalindrome(123321) << std::endl;
}
