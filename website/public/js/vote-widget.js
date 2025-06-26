class VoteWidget extends HTMLElement {
  constructor() {
    super();
    this.locked = false;
  }

  connectedCallback() {
    this.initializeElements();
    this.attachEventListeners();
    this.setupWebSocket();
  }

  initializeElements() {
    this.form = this.querySelector('#vote-form');
    this.buttons = this.form.querySelectorAll('button');
    this.countElements = {
      yay: this.querySelector('#yay-count'),
      may: this.querySelector('#may-count'),
      nay: this.querySelector('#nay-count')
    };
  }

  attachEventListeners() {
    this.form.addEventListener('submit', (e) => this.handleFormSubmit(e));
  }

  setupWebSocket() {
    const ws = new WebSocket(`ws://${location.host}/rpc`);
    ws.onmessage = (msg) => this.handleWebSocketMessage(msg);
  }

  handleFormSubmit(e) {
    e.preventDefault();
    
    if (this.locked) {
      this.animateButtons();
      return;
    }

    const payload = this.buildPayload(e);
    this.submitVote(payload);
  }

  buildPayload(e) {
    const formData = new FormData(this.form);
    const payload = Object.fromEntries(formData.entries());
    const label = e.submitter.querySelector('.label')?.textContent;
    
    payload.score = this.getScoreFromLabel(label);
    return payload;
  }

  getScoreFromLabel(label) {
    const scoreMap = { 'YAY': 1, 'MAY': 0, 'NAY': -1 };
    return scoreMap[label] ?? -1;
  }

  async submitVote(payload) {
    try {
      const response = await fetch('/vote', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(payload)
      });

      if (response.status === 201) {
        this.locked = true;
      }
      this.animateButtons();
    } catch (error) {
      console.error('Vote submission failed:', error);
      this.animateButtons();
    }
  }

  handleWebSocketMessage(msg) {
    try {
      const data = JSON.parse(msg?.data);
      
      if (data[0] === 'Create') {
        this.incrementCount(data[1].score);
      }
    } catch (error) {
      console.error('WebSocket message parsing failed:', error);
    }
  }

  incrementCount(score) {
    const countMap = {
      [-1]: this.countElements.nay,
      [0]: this.countElements.may,
      [1]: this.countElements.yay
    };

    const element = countMap[score];
    if (element) {
      const currentCount = parseInt(element.textContent, 10) || 0;
      element.textContent = currentCount + 1;
    }
  }

  animateButtons() {
    this.buttons.forEach(btn => {
      btn.classList.add('clicked');
      setTimeout(() => btn.classList.remove('clicked'), 300);
    });
  }

  updateCounts(counts) {
    if (!counts) return;

    Object.entries(counts).forEach(([key, value]) => {
      if (this.countElements[key] && typeof value !== 'undefined') {
        this.countElements[key].textContent = value;
      }
    });
  }

  updateRarity(percentage) {
    const rarityElement = this.querySelector('#rarity');
    if (rarityElement) {
      rarityElement.textContent = `${percentage}%`;
    }
  }
}

customElements.define('vote-widget', VoteWidget);