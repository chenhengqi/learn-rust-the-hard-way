class Solution {
public:
    int romanToInt(string s) {
        auto toInt = [](char c) {
            switch (c) {
            case 'I': return 1;
            case 'V': return 5;
            case 'X': return 10;
            case 'L': return 50;
            case 'C': return 100;
            case 'D': return 500;
            case 'M': return 1000;
            default: return 0;
            }
        };
        int num = 0;
        int prev = 0;
        int n = s.size();
        for (int i = 0; i < n; i++) {
            int curr = toInt(s[i]);
            if (curr > prev) {
                num += curr - 2 * prev;
            } else {
                num += curr;
            }
            prev = curr;
        }
        return num;
    }
};
