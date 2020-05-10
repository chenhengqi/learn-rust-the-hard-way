class Solution {
public:
    string countAndSay(int n) {
        string s = "1";
        for (int i = 1; i < n; i++) {
            s = say(s);
        }
        return s;
    }

    string say(const string& s) {
        string r;
        char c = s[0];
        int n = 1;
        for (int i = 1; i < s.size(); i++) {
            if (s[i] == c) {
                n++;
            } else {
                r += to_string(n) + string(1, c);
                c = s[i];
                n = 1;
            }
        }
        r += to_string(n) + string(1, c);
        return r;
    }
};
