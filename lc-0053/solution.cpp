class Solution {
public:
    int maxSubArray(vector<int>& nums) {
        int maxSum = numeric_limits<int>::min();
        int sum = 0;
        for (auto num : nums) {
            sum += num;
            maxSum = max(maxSum, sum);
            if (sum < 0) sum = 0;
        }
        return maxSum;
    }
};
