import pandas as pd
import matplotlib.pyplot as plt

# Load the CSV file
df = pd.read_csv('test_utils/spline_samples.csv')

# Create the plot
plt.figure(figsize=(12, 8))

# Plot the spline and its derivatives
plt.plot(df['x'], df['spline'], label='Spline', linewidth=2)
plt.plot(df['x'], df['derivative_1'], label='1st Derivative', linewidth=2)
plt.plot(df['x'], df['derivative_2'], label='2nd Derivative', linewidth=2)

# Customize the plot
plt.xlabel('x', fontsize=12)
plt.ylabel('y', fontsize=12)
plt.title('Spline and its Derivatives (0 to 3)', fontsize=14, fontweight='bold')
plt.legend(fontsize=11)
plt.grid(True, alpha=0.3)
plt.tight_layout()

# Save and show the plot
plt.savefig('test_utils/spline_plot.png', dpi=150)
print("Plot saved as test_utils/spline_plot.png")
plt.show()
