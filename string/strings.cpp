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
            //int length = 0;
            bool regist[astring.length()][astring.length()];
            for (int i = 0; i < astring.length(); ++i) {

                regist[i][i]=true;
            }
            for (int i= 0; i < astring.length()-1; ++i) {
                if (astring[i]==astring[i+1]) {
                    start=i,end=i+1;
                    regist[i][i+1]=true;
            //std::cout << start << " " << end << std::endl;
                }
            }
            //std::cout << start << " " << end << std::endl;
            for (int i= 2; i < astring.length(); ++i) {
                for (int j= 0; j < astring.length()-i; ++j) {
                    if (regist[j+1][i+j-1]==true && astring[j]==astring[i+j]) {
                        regist[j][i+j]=true;
                        start=j,end=i+j;
                        //std::cout << start << " " << end << std::endl;
                        //std::cout << astring[] << " " << end << std::endl;
                    }
                    
                } 
            }
            //std::cout << start << " " << end << std::endl;
            return astring.substr(start,end-start+1);
        }
    private:
        string astring;
};
int main()
{
    string a = "aabbaaabbcc";
    Solution *temp = new Solution(a);
    //temp->longestPalindrome();
    std::cout << temp->longestPalindrome() << std::endl;
}
