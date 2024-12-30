import pandas as pd
import seaborn as sns
import matplotlib.pyplot as plt

# Just for nicer-looking plots
sns.set_theme(style="whitegrid")

# Replace these entries with your own data
number_of_transfers = 1000
data_ = [{"n_workers":1,"chunk_size":50,"mean":222423840.12,"std_err":1166714.8018275609},{"n_workers":1,"chunk_size":100,"mean":220357527.11,"std_err":217100.2192492147},{"n_workers":1,"chunk_size":200,"mean":212448118.41,"std_err":671980.5403653897},{"n_workers":1,"chunk_size":400,"mean":210522743.76,"std_err":902677.0450659951},{"n_workers":2,"chunk_size":50,"mean":115365167.92,"std_err":148014.178180038},{"n_workers":2,"chunk_size":100,"mean":114248394.68,"std_err":101833.92536180337},{"n_workers":2,"chunk_size":200,"mean":110995009.6,"std_err":104030.82544193321},{"n_workers":2,"chunk_size":400,"mean":115038014.56,"std_err":402220.1047761249},{"n_workers":4,"chunk_size":50,"mean":63980705.0,"std_err":56511.701555789434},{"n_workers":4,"chunk_size":100,"mean":62056276.66,"std_err":75479.78031810478},{"n_workers":4,"chunk_size":200,"mean":61731941.66,"std_err":152948.82650142795},{"n_workers":4,"chunk_size":400,"mean":62429241.43,"std_err":378196.1583424578},{"n_workers":8,"chunk_size":50,"mean":43163489.16333331,"std_err":172824.82765375727},{"n_workers":8,"chunk_size":100,"mean":42428152.63999999,"std_err":569311.0545669408},{"n_workers":8,"chunk_size":200,"mean":40316746.10666668,"std_err":292783.3412754769},{"n_workers":8,"chunk_size":400,"mean":39174866.93000002,"std_err":440729.80948430306},{"n_workers":16,"chunk_size":50,"mean":55543671.655,"std_err":403185.85355601547},{"n_workers":16,"chunk_size":100,"mean":50672281.665,"std_err":63834.58990094246},{"n_workers":16,"chunk_size":200,"mean":49976485.55333334,"std_err":99656.93072208093},{"n_workers":16,"chunk_size":400,"mean":48059520.54333332,"std_err":178178.49575195537},{"n_workers":32,"chunk_size":50,"mean":75421341.26,"std_err":248037.34138481662},{"n_workers":32,"chunk_size":100,"mean":64387909.815,"std_err":169432.36116711586},{"n_workers":32,"chunk_size":200,"mean":58448654.795,"std_err":118160.75297754382},{"n_workers":32,"chunk_size":400,"mean":56497860.44,"std_err":617706.7580092071}]
data = [{"Workers": d["n_workers"], "Chunk Size": d["chunk_size"], "Mean (ms)": d["mean"] / 1e6, "Std Err (ms)": d["std_err"] / 1e6} for d in data_]

df = pd.DataFrame(data)

plt.figure(figsize=(10, 6))

# A pointplot or lineplot both work; pointplot can show some built-in error bars.
# We'll use pointplot with `errorbar=("se", 1)` so that it uses standard error from the data if possible.
# However, note that Seaborn's built-in aggregator might not use *our* "Std Err (ms)" column directly.
# If you want to pass your own errors exactly, you’ll need a more custom approach (see comment below).

sns.pointplot(
    data=df, 
    x="Chunk Size", 
    y="Mean (ms)",
    hue="Workers", 
    dodge=True, 
    join=True, 
    markers="o",
    # errorbar=("se", 1)  # Let Seaborn compute the standard error from repeated data, if you had them
    # To forcibly use your 'Std Err (ms)', you'd do a custom approach with e.g. plt.errorbar()
)

plt.title("Time to Execute 1000 Transfers by Workers and Chunk Size")
plt.xlabel("Chunk Size")
plt.ylabel("Mean Execution Time (ms)")
plt.legend(title="Workers")
plt.savefig("./viz/blockifier_benches.png")

plt.figure(figsize=(10, 6))

# First draw lines/markers without any aggregated error bars:
sns.lineplot(
    data=df, 
    x="Chunk Size", 
    y="Mean (ms)", 
    hue="Workers",
    marker="o"
)

# Now manually add error bars:
for _, row in df.iterrows():
    plt.errorbar(
        x=row["Chunk Size"], 
        y=row["Mean (ms)"],
        yerr=row["Std Err (ms)"], 
        capsize=3, 
        color=sns.color_palette()[int(row["Workers"]) % 10]  # Just pick color by Worker group
    )

plt.title("Time to Execute 1000 Transfers by Workers and Chunk Size (with manual error bars)")
plt.xlabel("Chunk Size")
plt.ylabel("Mean Execution Time (ms)")
plt.savefig("./viz/blockifier_benches_manual_errors.png")

plt.figure(figsize=(8, 6))

# Pivot so that each row is 'Workers' and each column is 'Chunk Size'
heatmap_data = df.pivot(index="Workers", columns="Chunk Size", values="Mean (ms)")

# Draw a heatmap with labeled cells
sns.heatmap(
    heatmap_data, 
    annot=True,      # Show the actual mean time in each cell
    fmt=".1f",       # Format to 1 decimal
    cmap="YlGnBu"    # A colormap you like
)

plt.title(f"Blockifier Mean Execution Time (ms) for 1k Transfers")
plt.ylabel("Workers")
plt.xlabel("Chunk Size")
plt.savefig("./viz/blockifier_benches_heatmap.png")


