
#include <algorithm>
#include <vector>
int maxDistance1(std::vector<int> &nums1, std::vector<int> &nums2) {
  int res = 0;
  for (int i = 0; i < nums1.size(); ++i) {
    int left = i;
    int right = nums2.size() - 1;

    while (left < right) {
      int mid = int((left + right + 1) / 2);
      if (nums1[i] <= nums2[mid]) {
        left = mid;
      } else {
        right = mid - 1;
      }
    }
    res = std::max(res, left - i);
  }
  return res;
}
int maxDistance2(std::vector<int> &nums1, std::vector<int> &nums2) {
  int res = 0;
  int i = 0;
  int j = 0;
  while (i < nums1.size() && j < nums2.size()) {
    if (nums1[i] <= nums2[j]) {
      res = std::max(res, j - i);
      j += 1;
    } else {
      i += 1;
      j = std::max(i, j);
    }
  }
  return res;
}

// nums1 = [9820, 8937, 7936, 4855, 4830, 4122, 2327, 1342, 1167, 815, 414]
// nums2 = [9889, 9817, 9800, 9777, 9670, 9646, 9304, 8977, 8974, 8802, 8626, 8622, 8456]
// nums1 = [55, 30, 5, 4, 2]
// nums2 = [100, 20, 10, 10, 5]
