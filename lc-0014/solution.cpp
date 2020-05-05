class Solution {
public:
    string longestCommonPrefix(vector<string>& strs) {
        if (strs.empty()) return "";
        int n = strs.size();
        if (n == 1) return strs[0];
        string prefix = strs[0];
        for (int i = 0; i < prefix.size(); i++) {
            for (int j = 1; j < n; j++) {
                if (strs[j].size() <= i) {
                    return string(prefix.begin(), prefix.begin() + i);
                }
                if (strs[j][i] != prefix[i]) {
                    return string(prefix.begin(), prefix.begin() + i);
                }
            }
        }
        return prefix;
    }
};
