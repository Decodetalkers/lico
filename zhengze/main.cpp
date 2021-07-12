#include <iostream>
#include <string>
using namespace std;
namespace Solution {
    bool isMatch(string s,string p){
        for(int i=0,j=0;i<s.length() || j<p.length();i++){
            switch (p[j]) {
                case '.':
                    j++;
                    break;
                case '*':
                    if (j-1>=0 && p[j-1]=='.') {
                        if (i+1>=p.length()) {
                            return true;
                        } else {
                            j++;
                            bool yes=false;
                            do {
                                i++;
                                if (s[i]==p[j]) {
                                    yes=true;
                                }
                            }while (i<s.length());
                            if (!yes) {
                                return false;
                            }
                        }
                    }
                    else if(j-1>=0 && p[j-1]!='.' && p[j-1]!='*') {
                        if (s[i]!=p[j-1]) {
                            j++;
                            break;
                        }
                        if(j==p.length()-1&&i==s.length()-1)
                            return true;
                        while (i+1<s.length() && s[i+1]==p[j-1]) {
                            i++;
                            std::cout << s[i] << std::endl;
                        }
                        j++;
                    }else {
                        throw "Illigule";
                    }
                    break;
                default:
                    if (s[i]==p[j]) {
                        j++;
                    }else {
                        return false;
                    }
                    break;
            }
            if (j==p.length()&&i<s.length()) {
                return false;
            }
        }
        return true;
    }
}
int main()
{
    try {
        std::cout<<Solution::isMatch("aaaacaaacbb", "a*ca*cb*")<<std::endl;
    }catch(const char* msg) {
        std::cerr << msg <<std::endl;
    }
}
