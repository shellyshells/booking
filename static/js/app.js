// =============================================================================
// ROOM RESERVATION SYSTEM - Frontend Application
// =============================================================================

class App {
    constructor() {
        this.currentView = 'dashboard';
        this.rooms = [];
        this.reservations = [];
        this.statistics = null;
        this.allLogs = [];
        
        this.init();
    }

    // Initialize the application
    async init() {
        this.setupNavigation();
        this.setupEventListeners();
        await this.loadData();
        this.showView('dashboard');
    }

    // Setup navigation click handlers
    setupNavigation() {
        document.querySelectorAll('.nav-item').forEach(item => {
            item.addEventListener('click', (e) => {
                e.preventDefault();
                const view = item.dataset.view;
                this.showView(view);
            });
        });
    }

    // Setup form and filter event listeners
    setupEventListeners() {
        // Booking form
        document.getElementById('booking-form')?.addEventListener('submit', (e) => {
            e.preventDefault();
            this.createReservation();
        });

        // Room selection in booking form
        document.getElementById('book-room')?.addEventListener('change', (e) => {
            this.updateRoomPreview(e.target.value);
        });

        // Global search bar
        const globalSearch = document.getElementById('global-search');
        if (globalSearch) {
            globalSearch.addEventListener('input', (e) => this.handleGlobalSearch(e.target.value));
            globalSearch.addEventListener('keydown', (e) => {
                if (e.key === 'Enter') {
                    e.preventDefault();
                    this.handleGlobalSearch(e.target.value);
                }
            });
        }

        // Filters
        document.getElementById('room-type-filter')?.addEventListener('change', () => this.filterRooms());
        document.getElementById('room-floor-filter')?.addEventListener('change', () => this.filterRooms());
        document.getElementById('room-capacity-filter')?.addEventListener('input', () => this.filterRooms());
        
        document.getElementById('reservation-status-filter')?.addEventListener('change', () => this.filterReservations());
        document.getElementById('reservation-date-filter')?.addEventListener('change', () => this.filterReservations());

        // Log level filter
        document.getElementById('log-level-filter')?.addEventListener('change', () => this.filterLogs());

        // Set default date to today
        const today = new Date().toISOString().split('T')[0];
        const dateInput = document.getElementById('book-date');
        if (dateInput) dateInput.value = today;
    }

    // Handle global search
    handleGlobalSearch(query) {
        const searchTerm = query.toLowerCase().trim();
        
        if (!searchTerm) {
            // If empty, show appropriate view
            if (this.currentView === 'rooms') {
                this.renderRooms();
            } else if (this.currentView === 'reservations') {
                this.renderReservations();
            }
            return;
        }

        // Search rooms
        const matchedRooms = this.rooms.filter(room => 
            room.name.toLowerCase().includes(searchTerm) ||
            room.room_type.toLowerCase().includes(searchTerm) ||
            (room.equipment && room.equipment.some(e => e.toLowerCase().includes(searchTerm)))
        );

        // Search reservations
        const matchedReservations = this.reservations.filter(res => {
            const room = this.rooms.find(r => r.id === res.room_id);
            return res.user_name.toLowerCase().includes(searchTerm) ||
                   res.user_email.toLowerCase().includes(searchTerm) ||
                   res.purpose.toLowerCase().includes(searchTerm) ||
                   (room && room.name.toLowerCase().includes(searchTerm));
        });

        // Navigate to appropriate view and show results
        if (matchedRooms.length > 0 && this.currentView !== 'reservations') {
            this.showView('rooms');
            this.renderRooms(matchedRooms);
        } else if (matchedReservations.length > 0) {
            this.showView('reservations');
            this.renderReservations(matchedReservations);
        } else if (matchedRooms.length === 0 && matchedReservations.length === 0) {
            this.showToast('No results found', 'warning');
        }
    }

    // Load all data from API
    async loadData() {
        try {
            const [roomsRes, reservationsRes, statsRes] = await Promise.all([
                fetch('/api/rooms'),
                fetch('/api/reservations'),
                fetch('/api/statistics')
            ]);

            const roomsData = await roomsRes.json();
            const reservationsData = await reservationsRes.json();
            const statsData = await statsRes.json();

            this.rooms = roomsData.data || [];
            this.reservations = reservationsData.data || [];
            this.statistics = statsData.data;

            this.updateUI();
        } catch (error) {
            console.error('Error loading data:', error);
            this.showToast('Failed to load data', 'error');
        }
    }

    // Show a specific view
    showView(viewName) {
        // Update navigation
        document.querySelectorAll('.nav-item').forEach(item => {
            item.classList.toggle('active', item.dataset.view === viewName);
        });

        // Update view visibility
        document.querySelectorAll('.view').forEach(view => {
            view.classList.remove('active');
        });
        document.getElementById(`${viewName}-view`)?.classList.add('active');

        // Update header
        const titles = {
            'dashboard': ['Dashboard', 'Overview of your room reservation system'],
            'rooms': ['Rooms', 'Manage all available rooms'],
            'reservations': ['Reservations', 'View and manage bookings'],
            'book': ['Book a Room', 'Create a new reservation'],
            'patterns': ['Design Patterns', 'Explore implemented patterns'],
            'logs': ['System Logs', 'View application logs']
        };

        const [title, subtitle] = titles[viewName] || ['', ''];
        document.getElementById('page-title').textContent = title;
        document.getElementById('page-subtitle').textContent = subtitle;

        this.currentView = viewName;

        // Load view-specific data
        if (viewName === 'patterns') this.loadPatternsData();
        if (viewName === 'logs') this.loadLogs();
        if (viewName === 'book') this.populateRoomSelect();
    }

    // Update all UI elements
    updateUI() {
        this.updateDashboard();
        this.renderRooms();
        this.renderReservations();
    }

    // Update dashboard statistics
    updateDashboard() {
        if (!this.statistics) return;

        document.getElementById('stat-total-rooms').textContent = this.statistics.total_rooms;
        document.getElementById('stat-available-rooms').textContent = this.statistics.available_rooms;
        document.getElementById('stat-reservations').textContent = this.statistics.total_reservations;
        document.getElementById('stat-today-bookings').textContent = this.statistics.today_bookings;

        this.renderRoomsByTypeChart();
        this.renderRecentActivity();
        this.renderQuickRooms();
    }

    // Render rooms by type chart
    renderRoomsByTypeChart() {
        const container = document.getElementById('rooms-by-type-chart');
        if (!container || !this.statistics?.rooms_by_type) return;

        const types = this.statistics.rooms_by_type;
        const maxValue = Math.max(...Object.values(types), 1);
        const colors = {
            'Conference': '#3b82f6',
            'Meeting': '#10b981',
            'Training': '#8b5cf6',
            'Auditorium': '#f59e0b',
            'Private Office': '#ec4899'
        };

        container.innerHTML = Object.entries(types).map(([type, count]) => `
            <div class="chart-bar">
                <div class="chart-bar-value">${count}</div>
                <div class="chart-bar-fill" style="height: ${(count / maxValue) * 120}px; background: ${colors[type] || '#6366f1'}"></div>
                <div class="chart-bar-label">${type}</div>
            </div>
        `).join('');
    }

    // Render recent activity
    renderRecentActivity() {
        const container = document.getElementById('recent-activity');
        if (!container) return;

        const recentReservations = this.reservations
            .sort((a, b) => new Date(b.created_at) - new Date(a.created_at))
            .slice(0, 5);

        if (recentReservations.length === 0) {
            container.innerHTML = '<p class="placeholder-text">No recent activity</p>';
            return;
        }

        container.innerHTML = recentReservations.map(res => {
            const room = this.rooms.find(r => r.id === res.room_id);
            const icon = res.status === 'Cancelled' ? '❌' : res.status === 'Confirmed' ? '✅' : '📅';
            const time = new Date(res.created_at).toLocaleString();
            
            return `
                <div class="activity-item">
                    <div class="activity-icon">${icon}</div>
                    <div class="activity-content">
                        <div class="activity-text">
                            <strong>${res.user_name}</strong> booked <strong>${room?.name || 'Room'}</strong>
                        </div>
                        <div class="activity-time">${time}</div>
                    </div>
                </div>
            `;
        }).join('');
    }

    // Render quick room overview
    renderQuickRooms() {
        const container = document.getElementById('quick-rooms');
        if (!container) return;

        const quickRooms = this.rooms.slice(0, 6);
        container.innerHTML = quickRooms.map(room => this.createRoomCard(room)).join('');
    }

    // Render rooms list
    renderRooms(rooms = this.rooms) {
        const container = document.getElementById('rooms-list');
        if (!container) return;

        if (rooms.length === 0) {
            container.innerHTML = '<p class="placeholder-text">No rooms found</p>';
            return;
        }

        container.innerHTML = rooms.map(room => this.createRoomCard(room)).join('');
    }

    // Create room card HTML
    createRoomCard(room) {
        const typeClass = room.room_type.toLowerCase().replace(' ', '-');
        const equipment = room.equipment?.slice(0, 4) || [];
        
        return `
            <div class="room-card">
                <div class="room-card-header">
                    <span class="room-type-badge ${typeClass}">${room.room_type}</span>
                    <span>${room.is_available ? '✅' : '🔒'}</span>
                </div>
                <div class="room-card-body">
                    <h4 class="room-name">${room.name}</h4>
                    <div class="room-meta">
                        <span>👥 ${room.capacity} people</span>
                        <span>🏢 Floor ${room.floor}</span>
                    </div>
                    <div class="room-equipment">
                        ${equipment.map(e => `<span class="equipment-tag">${this.getEquipmentEmoji(e)} ${e}</span>`).join('')}
                    </div>
                </div>
                <div class="room-card-footer">
                    <div class="room-price">€${room.hourly_rate}<span>/hour</span></div>
                    <button class="btn btn-primary btn-sm" onclick="app.quickBook('${room.id}')">
                        Book Now
                    </button>
                </div>
            </div>
        `;
    }

    // Get emoji for equipment
    getEquipmentEmoji(equipment) {
        const emojis = {
            'Projector': '📽️',
            'Whiteboard': '📋',
            'Video Conference': '📹',
            'Sound System': '🔊',
            'Air Conditioning': '❄️',
            'WiFi': '📶',
            'Computer': '💻',
            'Phone': '☎️',
            'Microphone': '🎤'
        };
        return emojis[equipment] || '✓';
    }

    // Filter rooms based on selected criteria
    filterRooms() {
        const typeFilter = document.getElementById('room-type-filter')?.value;
        const floorFilter = document.getElementById('room-floor-filter')?.value;
        const capacityFilter = document.getElementById('room-capacity-filter')?.value;

        let filtered = this.rooms.filter(room => {
            if (typeFilter && room.room_type !== typeFilter) return false;
            if (floorFilter && room.floor !== parseInt(floorFilter)) return false;
            if (capacityFilter && room.capacity < parseInt(capacityFilter)) return false;
            return true;
        });

        this.renderRooms(filtered);
    }

    // Render reservations table
    renderReservations(reservations = this.reservations) {
        const tbody = document.getElementById('reservations-table-body');
        if (!tbody) return;

        if (reservations.length === 0) {
            tbody.innerHTML = '<tr><td colspan="6" class="placeholder-text">No reservations found</td></tr>';
            return;
        }

        tbody.innerHTML = reservations.map(res => {
            const room = this.rooms.find(r => r.id === res.room_id);
            const startDate = new Date(res.start_time);
            const endDate = new Date(res.end_time);
            const statusClass = res.status.toLowerCase().replace(' ', '-');

            return `
                <tr>
                    <td><strong>${room?.name || 'Unknown'}</strong></td>
                    <td>
                        <div>${res.user_name}</div>
                        <small style="color: var(--text-muted)">${res.user_email}</small>
                    </td>
                    <td>
                        <div>${startDate.toLocaleDateString()}</div>
                        <small style="color: var(--text-muted)">${startDate.toLocaleTimeString([], {hour: '2-digit', minute:'2-digit'})} - ${endDate.toLocaleTimeString([], {hour: '2-digit', minute:'2-digit'})}</small>
                    </td>
                    <td>${res.attendees}</td>
                    <td><span class="status-badge ${statusClass}">${res.status}</span></td>
                    <td>
                        ${res.status === 'Pending' ? `
                            <button class="btn btn-success btn-sm" onclick="app.confirmReservation('${res.id}')">Confirm</button>
                        ` : ''}
                        ${res.status === 'Confirmed' ? `
                            <button class="btn btn-primary btn-sm" onclick="app.checkIn('${res.id}')">Check In</button>
                        ` : ''}
                        ${!['Cancelled', 'Completed'].includes(res.status) ? `
                            <button class="btn btn-danger btn-sm" onclick="app.cancelReservation('${res.id}')">Cancel</button>
                        ` : ''}
                    </td>
                </tr>
            `;
        }).join('');
    }

    // Filter reservations
    filterReservations() {
        const statusFilter = document.getElementById('reservation-status-filter')?.value;
        const dateFilter = document.getElementById('reservation-date-filter')?.value;

        let filtered = this.reservations.filter(res => {
            if (statusFilter && res.status !== statusFilter) return false;
            if (dateFilter) {
                const resDate = new Date(res.start_time).toISOString().split('T')[0];
                if (resDate !== dateFilter) return false;
            }
            return true;
        });

        this.renderReservations(filtered);
    }

    // Populate room select in booking form
    populateRoomSelect() {
        const select = document.getElementById('book-room');
        if (!select) return;

        select.innerHTML = '<option value="">Select a room...</option>' +
            this.rooms
                .filter(r => r.is_available)
                .map(room => `<option value="${room.id}">${room.name} (${room.room_type} - ${room.capacity} people)</option>`)
                .join('');
    }

    // Update room preview in booking form
    updateRoomPreview(roomId) {
        const container = document.getElementById('selected-room-preview');
        if (!container) return;

        const room = this.rooms.find(r => r.id === roomId);
        if (!room) {
            container.innerHTML = '<p class="placeholder-text">Select a room to see details</p>';
            return;
        }

        container.innerHTML = `
            <div class="room-preview">
                <h4 style="margin-bottom: 12px">${room.name}</h4>
                <div class="room-meta" style="margin-bottom: 16px">
                    <span class="room-type-badge ${room.room_type.toLowerCase()}">${room.room_type}</span>
                </div>
                <div style="margin-bottom: 8px">👥 Capacity: <strong>${room.capacity}</strong> people</div>
                <div style="margin-bottom: 8px">🏢 Floor: <strong>${room.floor}</strong></div>
                <div style="margin-bottom: 16px">💰 Rate: <strong>€${room.hourly_rate}/hour</strong></div>
                <div><strong>Equipment:</strong></div>
                <div class="room-equipment" style="margin-top: 8px">
                    ${(room.equipment || []).map(e => `<span class="equipment-tag">${this.getEquipmentEmoji(e)} ${e}</span>`).join('')}
                </div>
            </div>
        `;
    }

    // Quick book - navigate to booking with room preselected
    quickBook(roomId) {
        this.showView('book');
        setTimeout(() => {
            const select = document.getElementById('book-room');
            if (select) {
                select.value = roomId;
                this.updateRoomPreview(roomId);
            }
        }, 100);
    }

    // Create a new reservation
    async createReservation() {
        const roomId = document.getElementById('book-room')?.value;
        const userName = document.getElementById('book-name')?.value;
        const userEmail = document.getElementById('book-email')?.value;
        const date = document.getElementById('book-date')?.value;
        const startTime = document.getElementById('book-start')?.value;
        const endTime = document.getElementById('book-end')?.value;
        const attendees = document.getElementById('book-attendees')?.value;
        const purpose = document.getElementById('book-purpose')?.value;
        const userRole = document.getElementById('book-role')?.value;

        if (!roomId || !userName || !userEmail || !date || !startTime || !endTime) {
            this.showToast('Please fill in all required fields', 'error');
            return;
        }

        const startDateTime = new Date(`${date}T${startTime}:00Z`).toISOString();
        const endDateTime = new Date(`${date}T${endTime}:00Z`).toISOString();

        const payload = {
            room_id: roomId,
            user_name: userName,
            user_email: userEmail,
            start_time: startDateTime,
            end_time: endDateTime,
            attendees: parseInt(attendees) || 1,
            purpose: purpose || null,
            user_role: userRole
        };

        try {
            const response = await fetch('/api/reservations', {
                method: 'POST',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify(payload)
            });

            const data = await response.json();

            if (data.success && data.data?.success) {
                this.showToast('Reservation created successfully!', 'success');
                await this.loadData();
                this.showView('reservations');
                document.getElementById('booking-form')?.reset();
                const today = new Date().toISOString().split('T')[0];
                document.getElementById('book-date').value = today;
            } else {
                const errors = data.data?.validation?.errors || ['Failed to create reservation'];
                const warnings = data.data?.validation?.warnings || [];
                let message = errors.join(', ');
                if (warnings.length > 0) {
                    message += ' | Warnings: ' + warnings.join(', ');
                }
                this.showToast(message, 'error');
            }
        } catch (error) {
            console.error('Error creating reservation:', error);
            this.showToast('Failed to create reservation', 'error');
        }
    }

    // Cancel a reservation
    async cancelReservation(id) {
        if (!confirm('Are you sure you want to cancel this reservation?')) return;

        try {
            const response = await fetch(`/api/reservations/${id}/cancel`, { method: 'POST' });
            const data = await response.json();

            if (data.success) {
                this.showToast('Reservation cancelled', 'success');
                await this.loadData();
            } else {
                this.showToast(data.error || 'Failed to cancel reservation', 'error');
            }
        } catch (error) {
            console.error('Error cancelling reservation:', error);
            this.showToast('Failed to cancel reservation', 'error');
        }
    }

    // Confirm a reservation
    async confirmReservation(id) {
        try {
            const response = await fetch(`/api/reservations/${id}/confirm`, { method: 'POST' });
            const data = await response.json();

            if (data.success) {
                this.showToast('Reservation confirmed', 'success');
                await this.loadData();
            } else {
                this.showToast(data.error || 'Failed to confirm reservation', 'error');
            }
        } catch (error) {
            console.error('Error confirming reservation:', error);
            this.showToast('Failed to confirm reservation', 'error');
        }
    }

    // Check in to a reservation
    async checkIn(id) {
        try {
            const response = await fetch(`/api/reservations/${id}/checkin`, { method: 'POST' });
            const data = await response.json();

            if (data.success) {
                this.showToast('Checked in successfully', 'success');
                await this.loadData();
            } else {
                this.showToast(data.error || 'Failed to check in', 'error');
            }
        } catch (error) {
            console.error('Error checking in:', error);
            this.showToast('Failed to check in', 'error');
        }
    }

    // Load patterns page data
    async loadPatternsData() {
        try {
            const [observersRes, strategiesRes] = await Promise.all([
                fetch('/api/observers'),
                fetch('/api/validation-strategies')
            ]);

            const observersData = await observersRes.json();
            const strategiesData = await strategiesRes.json();

            this.renderObservers(observersData.data || []);
            this.renderStrategies(strategiesData.data || []);
        } catch (error) {
            console.error('Error loading patterns data:', error);
        }
    }

    // Render observers list
    renderObservers(observers) {
        const container = document.getElementById('observers-list');
        if (!container) return;

        const icons = {
            'Email Notifier': '📧',
            'Analytics Tracker': '📊',
            'Audit Logger': '📝',
            'Slack Notifier': '💬'
        };

        container.innerHTML = observers.map(obs => `
            <div class="observer-item">
                <div class="observer-icon">${icons[obs.name] || '👁️'}</div>
                <div class="observer-name">${obs.name}</div>
            </div>
        `).join('');
    }

    // Render strategies list
    renderStrategies(strategies) {
        const container = document.getElementById('strategies-list');
        if (!container) return;

        container.innerHTML = strategies.map(strat => `
            <div class="strategy-item">
                <div class="strategy-name">${strat.name}</div>
                <div class="strategy-desc">${strat.description}</div>
            </div>
        `).join('');
    }

    // Load system logs
    async loadLogs() {
        try {
            
            const response = await fetch('/api/logs?count=100');
            if (!response.ok) {
                throw new Error(`HTTP error! status: ${response.status}`);
            }
            const data = await response.json();
            
            this.allLogs = data.data || [];
            this.renderLogs(this.allLogs);
        } catch (error) {
            console.error('Error loading logs:', error);
            const container = document.getElementById('logs-list');
            if (container) {
                container.innerHTML = '<div class="log-entry"><span class="log-message">Error loading logs: ' + error.message + '</span></div>';
            }
        }
    }

    // Filter logs by type
    filterLogs() {
        const filterValue = document.getElementById('log-level-filter')?.value;
        
        if (!filterValue) {
            this.renderLogs(this.allLogs || []);
            return;
        }

        let filtered;
        if (filterValue === 'Error') {
            // Show only errors
            filtered = (this.allLogs || []).filter(log => log.is_error);
        } else {
            // Filter by action type
            filtered = (this.allLogs || []).filter(log => 
                log.action === filterValue
            );
        }
        this.renderLogs(filtered);
    }

    // Refresh logs
    refreshLogs() {
        this.loadLogs();
        this.showToast('Logs refreshed', 'info');
    }

    // Render logs
    renderLogs(logs) {
        const container = document.getElementById('logs-list');
        if (!container) {
            return;
        }

        if (!logs || logs.length === 0) {
            container.innerHTML = '<div class="log-entry"><span class="log-message">No activity yet. Create or cancel a reservation to see logs.</span></div>';
            return;
        }

        container.innerHTML = logs.map(log => {
            const actionClass = log.is_error ? 'error' : log.action.toLowerCase().replace('_', '-');
            const actionIcon = this.getActionIcon(log.action);
            
            return `
                <div class="log-entry ${log.is_error ? 'error-entry' : ''}">
                    <span class="log-timestamp">${log.timestamp}</span>
                    <span class="log-action ${actionClass}">${actionIcon} ${log.action.replace(/_/g, ' ')}</span>
                    <span class="log-user">${log.user || 'System'}</span>
                    <span class="log-details">${log.details}</span>
                </div>
            `;
        }).join('');
    }

    // Get icon for action type
    getActionIcon(action) {
        const icons = {
            'RESERVATION_CREATED': '✅',
            'RESERVATION_CANCELLED': '❌',
            'RESERVATION_CONFIRMED': '✔️',
            'CHECK_IN': '🚪',
            'ROOM_CREATED': '🏠',
            'ERROR': '⚠️',
            'VALIDATION_ERROR': '⛔'
        };
        return icons[action] || '📋';
    }

    // Open create room modal
    openCreateRoomModal() {
        const modalTitle = document.getElementById('modal-title');
        const modalBody = document.getElementById('modal-body');

        modalTitle.textContent = 'Create New Room';
        modalBody.innerHTML = `
            <form id="create-room-form" class="form">
                <div class="form-group">
                    <label for="room-name">Room Name *</label>
                    <input type="text" id="room-name" required placeholder="Conference Room A">
                </div>
                <div class="form-row">
                    <div class="form-group">
                        <label for="room-type">Room Type *</label>
                        <select id="room-type" required>
                            <option value="Conference">Conference</option>
                            <option value="Meeting">Meeting</option>
                            <option value="Training">Training</option>
                            <option value="Auditorium">Auditorium</option>
                            <option value="PrivateOffice">Private Office</option>
                        </select>
                    </div>
                    <div class="form-group">
                        <label for="room-capacity">Capacity *</label>
                        <input type="number" id="room-capacity" min="1" required value="10">
                    </div>
                </div>
                <div class="form-row">
                    <div class="form-group">
                        <label for="room-floor">Floor *</label>
                        <input type="number" id="room-floor" required value="1">
                    </div>
                    <div class="form-group">
                        <label for="room-rate">Hourly Rate (€)</label>
                        <input type="number" id="room-rate" step="0.01" value="25.00">
                    </div>
                </div>
                <button type="submit" class="btn btn-primary btn-block">Create Room</button>
            </form>
        `;

        document.getElementById('create-room-form').addEventListener('submit', (e) => {
            e.preventDefault();
            this.createRoom();
        });

        this.openModal();
    }

    // Create a new room
    async createRoom() {
        const payload = {
            name: document.getElementById('room-name')?.value,
            room_type: document.getElementById('room-type')?.value,
            capacity: parseInt(document.getElementById('room-capacity')?.value) || 10,
            floor: parseInt(document.getElementById('room-floor')?.value) || 1,
            hourly_rate: parseFloat(document.getElementById('room-rate')?.value) || 25.0
        };

        try {
            const response = await fetch('/api/rooms', {
                method: 'POST',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify(payload)
            });

            const data = await response.json();

            if (data.success) {
                this.showToast('Room created successfully!', 'success');
                this.closeModal();
                await this.loadData();
            } else {
                this.showToast(data.error || 'Failed to create room', 'error');
            }
        } catch (error) {
            console.error('Error creating room:', error);
            this.showToast('Failed to create room', 'error');
        }
    }

    // Modal functions
    openModal() {
        document.getElementById('modal')?.classList.add('active');
    }

    closeModal() {
        document.getElementById('modal')?.classList.remove('active');
    }

    // Show toast notification
    showToast(message, type = 'info') {
        const container = document.getElementById('toast-container');
        if (!container) return;

        const toast = document.createElement('div');
        toast.className = `toast ${type}`;
        
        const icons = {
            'success': '✅',
            'error': '❌',
            'warning': '⚠️',
            'info': 'ℹ️'
        };

        toast.innerHTML = `
            <span>${icons[type] || 'ℹ️'}</span>
            <span>${message}</span>
        `;

        container.appendChild(toast);

        setTimeout(() => {
            toast.style.animation = 'toastIn 0.3s ease reverse';
            setTimeout(() => toast.remove(), 300);
        }, 4000);
    }
}

// Initialize app when DOM is ready
const app = new App();
