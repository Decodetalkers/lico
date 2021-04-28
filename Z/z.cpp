#include <iostream>
#include <string>
#include <memory>
using namespace std;
class Solution {
    public:
        Solution(string astring){
            this->astring=astring;
        }
        string convert(int numRows){
            if (numRows==1) {
                return astring;
            }
            string v[numRows];
            for (int i = 0; i < astring.length(); ++i) {
                int m = i%(2*numRows-2);
                if (m<numRows) {
                   v[m].push_back(astring[i]);
                }else {
                   m=2*numRows-m-2;
                   for (int j = 0; j < m; ++j) {
                       v[j].push_back(' ');
                       std::cout << "test" << std::endl;
                   }
                   v[m].push_back(astring[i]);
                   for (int j = m+1; j < numRows; ++j) {
                       v[j].push_back(' ');
                   }

                }
            }
            for (int i = 0; i < numRows; ++i) {
                std::cout << v[i] << std::endl;
            }
            string output;
            for (int i= 0; i < numRows; ++i) {
                output+=v[i];
            }
            return output;
        }
    private:
        string astring;
};
int main()
{
    unique_ptr<Solution> test(new Solution("abcdefghi"));
    std::cout << test->convert(2) << std::endl;
}
