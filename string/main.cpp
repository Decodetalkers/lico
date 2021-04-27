#include <vector>
#include <iostream>
using namespace std;
class Solution {
    public:
        Solution(vector<int> nums1, vector<int> nums2){
            this->nums1=nums1;
            this->nums2=nums2;
        }
        double findMedianSortedArrays() {
            //vector<double> output;
            //int j=0;
            //for(int i=0;i<nums1.size();++i){
            //    while(j<nums2.size()&&nums2[j]<nums1[i]) {
            //        output.push_back(nums2[j]);
            //        j++;
            //    }
            //    output.push_back(nums1[i]);
            //}
            //while (j<nums2.size()){
            //    output.push_back(nums2[j]);
            //    j++;
            //};
            //for (int i = 0; i < output.size(); ++i) {
            //    cout<<output[i]<<endl;
            //}
            //if(output.size()%2==0)
            //    return (output[output.size()/2]+output[output.size()/2-1])/2;
            //else
            //    return output[output.size()/2];
            int length = nums1.size()+nums2.size();
            int position1,position2;
            int position=-1,j=0;
            double output1,output2;
            //double output;
            if (length%2 ==1) {
                position1=length/2;
                position2=length/2;
            } else {
                position1=length/2-1;
                position2=length/2;
            }
            //std::cout << position1<<"  "<<position2 << std::endl;
            for (int i = 0; i < nums1.size(); ++i) {
                while(j<nums2.size()&&nums2[j]<nums1[i]) {
                    position++;

                    std::cout << "test" << std::endl;
                    if (position==position1) {
                        output1 = (double)nums2[j];
                        //std::cout << output1 << std::endl;
                    }
                    if (position==position2) {
                        output2 = (double)nums2[j];
                        std::cout << output2 << std::endl;
                        return (output1+output2)/2;
                    }

                    j++;
                }
                position++;
                //std::cout << position << std::endl;
                if (position==position1) {
                    output1 = (double)nums1[i];
                    //std::cout << output1 <<"ss"<< std::endl;
                }
                if (position==position2) {
                    output2 = (double)nums1[i];
                    //std::cout << output2 <<"mm"<< std::endl;
                    return (output1+output2)/2;
                }
            }
            //std::cout << "Positon1= "<< position1 << std::endl;
            while (j<nums2.size()){
                position++;
                if (position==position1) {
                    output1 = (double)nums2[j];
                }
                if (position==position2) {
                    output2 = (double)nums2[j];
                    //std::cout << output2 <<"mm"<<output1<< std::endl;
                    return (output1+output2)/2;
                }
                j++;
            };
            return -1;
        }
    private:
        std::vector<int> nums1;
        std::vector<int> nums2;
};
int main()
{
    vector<int> nums1;
    nums1.push_back(1);
    nums1.push_back(3);
    vector<int> nums2;
    nums2.push_back(2);
    Solution *test = new Solution(nums1,nums2);
    cout<<test->findMedianSortedArrays()<<endl;
}
