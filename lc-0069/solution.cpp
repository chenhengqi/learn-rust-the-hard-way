class Solution {
public:
    int mySqrt(int x) {
        if (x < 2) return x;
        long left = 0, right = x;
        while (left <= right) {
            long mid = left + (right - left) / 2;
            long square = mid * mid;
            if (square == x) {
                return mid;
            } else if (square > x) {
                right = mid - 1;
            } else {
                left = mid + 1;
            }
        }
        return left * left > x ? left - 1 : left;
    }
};
