class Solution {
public:
    bool isValid(string s) {
        auto isPair = [](char l, char r) {
            return (l == '(' && r == ')') || (l == '[' && r == ']') || (l == '{' && r == '}');
        };

        stack<char> t;
        for (auto c : s) {
            if (c == '(' || c == '[' || c == '{') {
                t.push(c);
            } else {
                if (t.empty()) return false;
                char l = t.top();
                t.pop();
                if (!isPair(l, c)) {
                    return false;
                }
            }
        }
        return t.empty();
    }
};
