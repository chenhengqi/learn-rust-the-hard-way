class Solution {
public:
    bool isPalindrome(int x) {
        if (!x) return true;
        if (x < 0 || x % 10 == 0) return false;
        string s = to_string(x);
        int l = 0, r = s.size() - 1;
        while (l < r) {
            if (s[l] == s[r]) {
                l++;
                r--;
            } else {
                return false;
            }
        }
        return true;
    }
};
