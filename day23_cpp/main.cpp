#include <iostream>

using namespace std;

struct ListNode
{
    int val;
    ListNode *next;
};

int main()
{
    string input = "467528193";
    const int NUM_MOVES = 10000000;

    ListNode *head = nullptr;
    ListNode *curr = nullptr;
    ListNode *prev = nullptr;
    ListNode **location_arr = new ListNode *[1000000 + 1];

    for (auto c : input)
    {
        curr = new ListNode();
        curr->val = c - '0';
        curr->next = nullptr;
        if (head == nullptr)
        {
            head = curr;
        }
        if (prev != nullptr)
            prev->next = curr;
        prev = curr;
        location_arr[curr->val] = curr;
    }
    for (int i = 10; i <= 1000000; i++)
    {
        prev = curr;
        curr = new ListNode();
        curr->val = i;
        curr->next = nullptr;
        prev->next = curr;
        location_arr[curr->val] = curr;
    }
    curr->next = head;

    curr = head;
    for (int i = 0; i < NUM_MOVES; i++)
    {
        // cout << "Move: " << i << endl;
        ListNode *three_ahead = curr->next->next->next;
        ListNode *split = curr->next;
        int vals[3] = {split->val, split->next->val, split->next->next->val};

        // adjust spacing
        curr->next = three_ahead->next;

        // take 3 cups
        three_ahead->next = nullptr;

        int target_label = curr->val;
        while (1)
        {
            target_label -= 1;
            if (target_label <= 0)
                target_label = 1000000; // wrap-around

            bool to_break = true;
            for (int i = 0; i < 3; i++)
            {
                if (vals[i] == target_label)
                {
                    to_break = false;
                    break;
                }
            }
            if (to_break)
            {
                break;
            }
        }

        ListNode *temp = location_arr[target_label];
        three_ahead->next = temp->next;
        temp->next = split;
        curr = curr->next;
    }

    // Part 1
    // string ans = "";
    // while (curr->val != 1)
    // {
    //     curr = curr->next;
    // }

    // curr = curr->next;
    // while (curr->val != 1)
    // {
    //     ans.push_back('0' + curr->val);
    //     curr = curr->next;
    // }

    long long ans = 1;

    while (curr->val != 1)
    {
        curr = curr->next;
    }

    ans *= curr->next->val;
    ans *= curr->next->next->val;

    cout << "Ans: " << ans << endl;
}