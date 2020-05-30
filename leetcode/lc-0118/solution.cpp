class Solution {
public:
    vector<vector<int>> generate(int numRows) {
        if (numRows == 0) return {};
        if (numRows == 1) return {{1}};
        if (numRows == 2) return {{1}, {1, 1}};
        vector<vector<int>> result;
        result.push_back({1});
        result.push_back({1, 1});
        for (int i = 2; i < numRows; i++) {
            vector<int> row{1};
            int n = result[i - 1].size() - 1;
            for (int j = 0; j < n; j++) {
                row.push_back(result[i - 1][j] + result[i - 1][j + 1]);
            }
            row.push_back(1);
            result.push_back(row);
        }
        return result;
    }
};
