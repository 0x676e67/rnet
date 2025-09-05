import matplotlib.pyplot as plt
import numpy as np


def _plot_main_sessions(df, main_sessions, sizes, stat_types, filename):
    """Plot main sessions (sync and async)"""
    num_sessions = len(main_sessions)
    # Allocate more height for each subplot to ensure sufficient spacing
    subplot_height = 8  # Fixed height for each subplot
    total_height = subplot_height * num_sessions + 2  # Extra 2 inches for spacing

    fig, axes = plt.subplots(
        num_sessions,
        1,
        figsize=(20, total_height),
        constrained_layout=False,  # Disable constrained_layout, use manual layout
    )

    if num_sessions == 1:
        axes = [axes]

    for idx, session in enumerate(main_sessions):
        ax = axes[idx]
        subdf = df[df["session"] == session]
        names = subdf["name"].unique()
        x = np.arange(len(names))
        width = 0.12

        max_height = 0

        for i, size in enumerate(sizes):
            for j, stat in enumerate(stat_types):
                vals = []
                for name in names:
                    v = subdf[(subdf["name"] == name) & (subdf["size"] == size)][stat]
                    vals.append(v.values[0] if not v.empty else 0)
                offset = (i * len(stat_types) + j) * width
                rects = ax.bar(x + offset, vals, width, label=f"{stat} {size}")
                ax.bar_label(rects, padding=2, fontsize=7, rotation=90)
                if vals:
                    max_height = max(max_height, max(vals))

        ax.set_xticks(x + (len(sizes) * len(stat_types) * width) / 2 - width / 2)
        ax.set_xticklabels(names, rotation=0, ha="center", fontsize=8)
        ax.set_ylabel("Time (s)")
        ax.set_title(f"Benchmark | {session}", fontsize=12, fontweight="bold")
        ax.legend(loc="upper left", ncol=3, prop={"size": 7})
        ax.tick_params(axis="x", labelsize=8)
        ax.grid(True, alpha=0.3)

        if max_height > 0:
            ax.set_ylim(0, max_height * 1.35)

    plt.subplots_adjust(
        hspace=0.5, top=0.95, bottom=0.1, left=0.08, right=0.98
    )  # Set explicit margins for all sides
    plt.savefig(filename, format="jpg", dpi=150, bbox_inches="tight")
    plt.show()


def _plot_threaded_sessions(df, threaded_sessions, sizes, stat_types, filename):
    """Plot threaded sessions separately"""
    threaded_df = df[df["session"].str.startswith("Threaded")].copy()
    thread_counts = sorted(threaded_df["threads"].unique())

    fig2, axes2 = plt.subplots(
        len(thread_counts),
        1,
        figsize=(20, 10 * len(thread_counts)),
        constrained_layout=False,  # Disable constrained_layout, use manual layout
    )

    if len(thread_counts) == 1:
        axes2 = [axes2]

    for idx, thread_count in enumerate(thread_counts):
        ax = axes2[idx]
        thread_df = threaded_df[threaded_df["threads"] == thread_count]

        # Get all unique session types for this thread count
        thread_session_types = thread_df["session"].unique()

        names = thread_df["name"].unique()
        x = np.arange(len(names))
        width = 0.08
        max_height = 0
        bar_index = 0

        # Plot each session type
        for session_type in thread_session_types:
            session_df = thread_df[thread_df["session"] == session_type]
            session_label = session_type.replace("Threaded-", "")

            for i, size in enumerate(sizes):
                for j, stat in enumerate(stat_types):
                    vals = []
                    for name in names:
                        v = session_df[
                            (session_df["name"] == name) & (session_df["size"] == size)
                        ][stat]
                        vals.append(v.values[0] if not v.empty else 0)
                    offset = bar_index * width
                    rects = ax.bar(
                        x + offset,
                        vals,
                        width,
                        label=f"{session_label} {stat} {size}",
                    )
                    ax.bar_label(rects, padding=2, fontsize=6, rotation=90)
                    if vals:
                        max_height = max(max_height, max(vals))
                    bar_index += 1

        ax.set_xticks(x + (bar_index * width) / 2 - width / 2)
        ax.set_xticklabels(names, rotation=0, ha="center", fontsize=8)
        ax.set_ylabel("Time (s)")
        ax.set_title(
            f"Benchmark | Threaded ({thread_count} threads)",
            fontsize=12,
            fontweight="bold",
        )
        ax.legend(loc="upper left", ncol=4, prop={"size": 6})
        ax.tick_params(axis="x", labelsize=8)
        ax.grid(True, alpha=0.3)

        if max_height > 0:
            ax.set_ylim(0, max_height * 1.35)

    threaded_filename = filename.replace(".jpg", "_threaded.jpg")
    plt.subplots_adjust(
        hspace=0.5, top=0.95, bottom=0.1, left=0.08, right=0.98
    )  # Set explicit margins for all sides
    plt.savefig(threaded_filename, format="jpg", dpi=150, bbox_inches="tight")
    plt.show()


def plot_benchmark_multi(df, filename):
    """
    Draw multi-subplot, multi-group, multi-metric bar charts for time/cpu_time/different payload sizes.
    Generate separate plots for sync/async and session/non-session combinations.
    """
    # Keep only necessary columns
    df = df[["name", "session", "threads", "size", "time", "cpu_time"]].copy()
    df["threads"] = df["threads"].fillna(1).astype(int)

    # Get unique session types
    existing_session_types = df["session"].unique()

    sizes = sorted(df["size"].unique(), key=lambda x: int(x.replace("k", "")))
    stat_types = ["time", "cpu_time"]

    # Separate main sessions (non-threaded) and threaded sessions
    main_sessions = [s for s in existing_session_types if not s.startswith("Threaded")]
    threaded_sessions = [s for s in existing_session_types if s.startswith("Threaded")]

    # Plot main sessions (sync and async)
    if main_sessions:
        _plot_main_sessions(df, main_sessions, sizes, stat_types, filename)

    # Plot threaded sessions separately
    if threaded_sessions:
        _plot_threaded_sessions(df, threaded_sessions, sizes, stat_types, filename)
