# 1855. Maximum Distance Between a Pair of Values
# Medium
# You are given two non-increasing 0-indexed integer arrays nums1​​​​​​ and nums2​​​​​​.

# A pair of indices (i, j), where 0 <= i < nums1.length and 0 <= j < nums2.length, is valid if both i <= j and nums1[i] <= nums2[j]. The distance of the pair is j - i​​​​.

# Return the maximum distance of any valid pair (i, j). If there are no valid pairs, return 0.

# An array arr is non-increasing if arr[i-1] >= arr[i] for every 1 <= i < arr.length.
from typing import List


def maxDistance1(nums1: List[int], nums2: List[int]) -> int:
    # use binary search
    res = 0
    for i in range(len(nums1)):
        left = i
        right = len(nums2) - 1
        while left < right:
            mid = int((left + right + 1) / 2)
            if nums1[i] <= nums2[mid]:
                left = mid
            else:
                right = mid - 1
        res = max(res, left - i)
    return res


def maxDistance2(nums1: List[int], nums2: List[int]) -> int:
    res = 0
    i = 0
    j = 0
    while (i < len(nums1)) & (j < len(nums2)):
        if nums1[i] <= nums2[j]:
            res = max(res, j - i)
            j += 1
        else:
            i += 1
            j = max(i, j)
    return res


nums1 = [9820, 8937, 7936, 4855, 4830, 4122, 2327, 1342, 1167, 815, 414]
nums2 = [9889, 9817, 9800, 9777, 9670, 9646, 9304, 8977, 8974, 8802, 8626, 8622, 8456]
nums1 = [55, 30, 5, 4, 2]
nums2 = [100, 20, 10, 10, 5]
