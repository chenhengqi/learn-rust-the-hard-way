class Solution {
public:
    vector<int> getRow(int rowIndex) {
        if (rowIndex == 0) return {1};
        if (rowIndex == 1) return {1, 1};
        if (rowIndex == 2) return {1, 2, 1};
        vector<int> current{1, 2, 1};
        vector<int> next;
        next.reserve(40);
        for (int i = 2; i < rowIndex; i++) {
            next.clear();
            next.push_back(1);
            for (int j = 0; j < i; j++) {
                next.push_back(current[j] + current[j + 1]);
            }
            next.push_back(1);
            swap(next, current);
        }
        return current;
    }
};
