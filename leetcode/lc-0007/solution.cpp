class Solution {
public:
    int reverse(int x) {
        long lx = x;
        bool negative = false;
        if (lx < 0) {
            negative = true;
            lx *= -1;
        }

        long rx = 0;
        while (lx) {
            rx = rx * 10 + lx % 10;
            lx /= 10;
            if (negative) {
                if (-rx < INT_MIN) return 0;
            } else {
                if (rx > INT_MAX) return 0;
            }
        }
        return negative ? -rx : rx;
    }
};
