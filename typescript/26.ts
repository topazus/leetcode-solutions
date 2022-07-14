function removeDuplicates(nums: number[]): number {
  let n = 1
  for (let i = 0; i < nums.length - 1; i++) {
    if (nums[i] != nums[i + 1]) {
      nums[n] = nums[i + 1]
      n += 1
    }
  }
  return n
}
