class Solution {
public:
    int lengthOfLastWord(string s) {
        bool in_word = false;
        int length = 0;
        for (auto it = s.rbegin(); it != s.rend(); it++) {
            if (*it == ' ') {
                if (in_word) break;
            } else {
                in_word = true;
                length++;
            }
        }
        return length;
    }
};
