#include <iostream>
#include <string>
using namespace std;
class Solution{
    public:
        Solution(string astring){
            this->astring=astring;
        }
        string longestPalindrome() {
            int start = 0, end = 0;
            int length = 0;
            if (astring.length()==1) {
                return astring;
            }
            for (int i = 0; i < astring.length(); ++i) {
                //std::cout << i << std::endl;
                //std::cout << astring[9]<<astring[11] << std::endl;
                if (i+1<astring.length() && astring[i+1]==astring[i]) {
                    int m = i-1, n = i+2;
                    int len =2;
                    while (m>=0 && n<astring.length()) {
                        if (astring[m]==astring[n]) {
                            //start=m,end=n;
                            len=n-m+1;
                            n+=1,m-=1;
                        }
                        else {
                            break;
                        }
                    }
                    if (len>length) {
                        start=m+1,end=n-1;
                        //std::cout << "one" <<start<<" "<< end << std::endl;
                        length=len;
                    }
                }
                if(i+1<astring.length()) {
                    int m = i-1,n=i+1;
                    int len=1;
                    while (m>=0 && n<astring.length()) {
                        if (astring[m]==astring[n]) {
                            len=n-m+1;
                            n+=1,m-=1;
                        }
                        else {
                            break;
                        }
                    }
                    if(len>length){
                        start=m+1,end=n-1;
                        //std::cout << start<<" "<< end << std::endl;
                        length=len;
                    }
                }
            }
            //std::cout << length << std::endl;
            return astring.substr(start,length);
        }
    private:
        string astring;
};
int main()
{
    string a = "a";
    Solution *temp = new Solution(a);
    std::cout << temp->longestPalindrome() << std::endl;
}
