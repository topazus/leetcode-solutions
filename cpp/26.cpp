#include <vector>
using namespace std;

class Solution {
 public:
  int removeDuplicates(vector<int>& nums) {
    auto n = 1;
    for (auto i = 1; i < nums.size(); i++) {
      if (nums[i] != nums[i - 1]) {
        nums[n] = nums[i];
        n++;
      }
    }
    return n;
  }
};

void main() {}
