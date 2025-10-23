<template>
    <div class="dashboard">
        <h1>Dashboard</h1>
        <div class="dashboard-content">
            <div class="dashboard-stats">
                <div class="stat-card">
                    <h3>Total Users</h3>
                    <p>100</p>
                </div>
                <div class="stat-card">
                    <h3>Active Tasks</h3>
                    <p>25</p>
                </div>
                <div class="stat-card">
                    <h3>Completed</h3>
                    <p>75</p>
                </div>
            </div>
        </div>
    </div>
</template>

<script setup>
// You can add your component logic here
</script>

<style scoped>
.dashboard {
    padding: 20px;
}

.dashboard-content {
    margin-top: 20px;
}

.dashboard-stats {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(250px, 1fr));
    gap: 20px;
}

.stat-card {
    background: #ffffff;
    border-radius: 8px;
    padding: 20px;
    box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
}

.stat-card h3 {
    margin: 0 0 10px 0;
    color: #666;
}

.stat-card p {
    margin: 0;
    font-size: 24px;
    font-weight: bold;
    
    color: #333;
}
</style>