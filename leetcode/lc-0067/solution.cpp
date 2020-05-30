class Solution {
public:
    string addBinary(string a, string b) {
        if (a == "0") return b;
        if (b == "0") return a;

        // 111 + 111 = 1011 => 4 digits
        int m = a.size();
        int n = b.size();
        int k = max(m, n) + 1;
        string result(k, '0');

        for (int i = m - 1, j = k - 1; i >= 0; i--, j--) {
            result[j] += (a[i] - '0');
        }
        for (int i = n - 1, j = k - 1; i >= 0; i--, j--) {
            result[j] += (b[i] - '0');
        }
        for (int i = k - 1; i > 0; i--) {
            int sum = result[i] - '0';
            result[i] = sum % 2 + '0';
            result[i - 1] += sum / 2;
        }

        for (int i = 0; i < k; i++) {
            if (result[i] != '0') return result.substr(i);
        }
        return result;
    }
};
