#include <cmath>
#include <cstdint>
#include <cstdlib>
#include <iostream>
#include <string.h>
class Change
{
private:
    int input;

public:
    Change(long input){
        if(input > INT32_MAX){
            throw "bigger than int";
        }
        this->input=input;
    };
    int reverse(){
        bool up = (input<0);
        int temp = std::abs(input);
        int output = 0;
        while (temp>0) {
            if (output*10/10 !=output) {
                return 0;
            }
            output=output*10;
            output+=temp%10;
            temp=temp/10;
        }
        if(up){
            output=-1*output;
        }
        return output;
    }
};
int main(int argc, char * argv[])
{  
    //std::cout<< INT32_MAX<<std::endl;
    //std::cout << argv[1] << std::endl;
    long input=1234;
    //if(argv[1]!=NULL){
    //    input = atol(argv[1]);
    //    std::cout << input << std::endl;
    //}
    if (argv[1]!=NULL) {
        if (strcmp(argv[1],"-h")==0) {
            std::cout << "You can enter a number" << std::endl;
            return 0;
        }
        else if (strcmp(argv[1], "-i")==0) {
            //std::cout <<  << std::endl;
            if (argv[2]!=NULL) {
                input = atol(argv[2]);
            }else {
                std::cout << "please input a number" << std::endl;
                return 0;
            }
        }
    }
    try{
        Change *test = new Change(input);
        std::cout << test->reverse() << std::endl;
    }catch(const char* msg){
        std::cerr << msg <<std::endl;
    }

}
