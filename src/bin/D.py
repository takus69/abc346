n = int(input())
s = input()
c = list(map(int, input().split()))

t = 1
b = []
sum_b = 0
a = []
sum_a = 0
for i in range(n):
    if s[i] == str(t):
        sum_b += c[i]
        b.append(sum_b)
        a.append(sum_a)
    else:
        sum_a += c[i]
        b.append(sum_b)
        a.append(sum_a)
    t = 1 - t
ans = sum_a + sum_b
for i in range(n-1):
    ans = min(ans, a[i]+b[-1]-b[i], b[i]+a[-1]-a[i])
print(ans)