<script>
  import { onMount } from "svelte";
  let goal = { current_objective: "Initializing...", available_ram: "..." };
  async function fetchGoal() {
    try {
      const res = await fetch("./neural_goal.json");
      goal = await res.json();
    } catch (e) { console.error("Pulse failed", e); }
  }
  onMount(() => {
    fetchGoal();
    setInterval(fetchGoal, 5000);
  });
</script>
<main style="background: #1a1a1a; color: #00ff41; padding: 20px; font-family: monospace;">
  <h1>{goal.node_id || "NODE"} ANALYTICS</h1>
  <p><strong>Objective:</strong> {goal.current_objective}</p>
  <p><strong>RAM:</strong> {goal.available_ram}</p>
</main>
