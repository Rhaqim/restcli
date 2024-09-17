const express = require('express');

const router = express.Router();

// Sample route for GET request
router.get('/', (req, res) => {
    res.send('Hello, World!');
});

// Sample route for POST request
router.post('/data', (req, res) => {
    const data = req.body;
    res.json({ message: 'Data received', data });
});

module.exports = router;